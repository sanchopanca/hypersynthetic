mod attributes;
mod generator;
mod nodes;
mod parser;
mod utils;

extern crate proc_macro;

use generator::generate_nodes;
use nodes::NodeCollection;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{ToTokens, quote};
use std::collections::HashSet;
use syn::{ItemFn, ext::IdentExt, parse_macro_input, visit::Visit, visit_mut::VisitMut};
use utils::{BUILD_METHOD, is_pascal_case};

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let parsed_html_nodes = parse_macro_input!(input as NodeCollection);
    let expanded = generate_nodes(parsed_html_nodes);
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    // `#[component(...)]`: there are no options, so arguments would be ignored silently
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "`#[component]` doesn't take arguments",
        )
        .to_compile_error()
        .into();
    }

    let mut function = parse_macro_input!(item as ItemFn);

    if let Err(err) = check_signature(&function.sig) {
        return err.to_compile_error().into();
    }
    let return_type = match return_type(&function.sig) {
        Ok(ty) => ty.clone(),
        Err(err) => return err.to_compile_error().into(),
    };

    // Props struct fields can't have elided lifetimes
    name_elided_lifetimes(&mut function);

    // Check if the function's identifier is PascalCase
    let fn_name = &function.sig.ident;
    if !is_pascal_case(fn_name) {
        return syn::Error::new(
            function.sig.ident.span(),
            "Component name must be in PascalCase",
        )
        .to_compile_error()
        .into();
    }

    // Extract visibility
    let vis = &function.vis;

    // Generate Props struct name
    let props_name = quote::format_ident!("{}Props", fn_name);
    let props_builder_name = quote::format_ident!("{}PropsBuilder", fn_name);

    // Every parameter is a typed one: `check_signature` refused `self`
    let params: Vec<_> = function
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                Some(pat_type)
            } else {
                None
            }
        })
        .collect();

    // Props become struct fields, so they need plain names (`mut n` → `n`).
    // The original patterns stay on the internal function.
    let param_names = match params
        .iter()
        .map(|param| prop_name(&param.pat))
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(names) => names,
        Err(err) => return err.to_compile_error().into(),
    };

    // Props become struct fields, where `impl Trait` isn't allowed
    if let Some(impl_trait) = params.iter().find_map(|param| find_impl_trait(&param.ty)) {
        return syn::Error::new_spanned(
            impl_trait,
            "component props can't use `impl Trait`, because they are struct fields: \
             use a generic parameter instead, like `<T: Trait>`",
        )
        .to_compile_error()
        .into();
    }

    if let Err(err) = check_children_prop(&params, &param_names) {
        return err.to_compile_error().into();
    }

    // Generate struct fields, with the parameter's `#[builder(...)]` attributes
    // (`default`, `setter(into)`, ...) for the TypedBuilder derive
    let struct_fields = params.iter().zip(&param_names).map(|(param, name)| {
        let ty = &param.ty;
        let builder_attrs: Vec<_> = param
            .attrs
            .iter()
            .filter(|attr| is_builder_attr(attr))
            .collect();
        // `children` is optional: without content between the tags, it's an empty
        // fragment. Unless the parameter configures its builder itself.
        let default =
            (is_children(name) && builder_attrs.is_empty()).then(|| quote! { #[builder(default)] });
        quote! {
            #default
            #(#builder_attrs)*
            #name: #ty
        }
    });

    // Generate the internal function name
    let internal_fn_name = quote::format_ident!("__{}", fn_name);

    // Clone the original function and rename it
    let mut internal_function = function.clone();
    internal_function.sig.ident = internal_fn_name.clone();
    internal_function.vis = syn::Visibility::Inherited;

    // `#[builder]` is only valid inside the derive, so it can't stay on the parameters
    for input in &mut internal_function.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            pat_type.attrs.retain(|attr| !is_builder_attr(attr));
        }
    }

    // Docs and deprecation describe the public component, so they move to the
    // wrapper. The rest (`#[allow]`, `#[inline]`, ...) is about the body and stays.
    // `#[cfg]` never gets here: the compiler evaluates it before calling the macro.
    let (wrapper_attrs, internal_attrs): (Vec<_>, Vec<_>) =
        std::mem::take(&mut internal_function.attrs)
            .into_iter()
            .partition(|attr| attr.path().is_ident("doc") || attr.path().is_ident("deprecated"));
    internal_function.attrs = internal_attrs;
    let props_doc = format!("Props for the [`{fn_name}`] component.");

    // Add allow directive for snake_case to the internal function
    let allow_attr: syn::Attribute = syn::parse_quote!(#[allow(non_snake_case)]);
    internal_function.attrs.push(allow_attr);

    // Extract lifetimes and generics from the updated internal function
    let generics = &internal_function.sig.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let call = call_spanned_as(&return_type, &internal_fn_name, &param_names);
    let wrapper_fn = quote! {
        #(#wrapper_attrs)*
        #[allow(non_snake_case)]
        #vis fn #fn_name #impl_generics(props: #props_name #ty_generics) -> ::hypersynthetic::HtmlFragment #where_clause {
            let #props_name { #(#param_names),* } = props;
            #call
        }
    };

    let build_method = syn::Ident::new(BUILD_METHOD, proc_macro2::Span::call_site());

    // Generate the final output - always generate Props struct
    let output = quote! {
        #[doc = #props_doc]
        #[derive(::hypersynthetic::__private::typed_builder::TypedBuilder)]
        // The generated builder code refers to typed_builder, which users don't
        // depend on directly
        #[builder(crate_module_path = ::hypersynthetic::__private::typed_builder)]
        // Not typed-builder's `build`, so that a prop can have that name
        #[builder(build_method(name = #build_method))]
        #vis struct #props_name #impl_generics #where_clause {
            #(#struct_fields,)*
        }

        impl #impl_generics ::hypersynthetic::component::Props for #props_name #ty_generics #where_clause {
            type Builder = #props_builder_name #ty_generics;

            fn builder() -> Self::Builder {
                #props_name::builder()
            }
        }

        #internal_function

        #wrapper_fn
    };

    output.into()
}

/// A component is a plain `fn`: not `const`, `async`, `unsafe` or `extern`, and not
/// a method.
///
/// `html!` calls the wrapper generated here, which is always a plain `fn`, and the
/// qualifiers would stay on the internal function it calls. That either doesn't
/// compile, with an error about generated code (`async`: "expected `HtmlFragment`,
/// found future"; `unsafe`: "call to unsafe function `__Card`"), or compiles and means
/// nothing (`extern "C"`, and a `const fn` whose body happens to be const).
///
/// A `self` parameter means the function is in an `impl` block, where the props
/// struct generated next to it isn't allowed: five errors about generated code.
/// Without `self`, a function in an `impl` block looks like any other from here, so
/// that case can't be caught.
///
/// All of them are reported at once, in the order they're written in.
fn check_signature(sig: &syn::Signature) -> syn::Result<()> {
    let errors = [
        sig.constness.map(|token| {
            syn::Error::new_spanned(
                token,
                "a component can't be a `const fn`, because an `HtmlFragment` can't be built \
                 at compile time: remove `const`",
            )
        }),
        sig.asyncness.map(|token| {
            syn::Error::new_spanned(
                token,
                "a component can't be an `async fn`, because `html!` calls it without \
                 `.await`: await what it needs before `html!` and pass the result as a prop",
            )
        }),
        sig.unsafety.map(|token| {
            syn::Error::new_spanned(
                token,
                "a component can't be an `unsafe fn`, because `html!` can't call it in an \
                 `unsafe` block: remove `unsafe` and write `unsafe { ... }` in the body where \
                 it's needed",
            )
        }),
        sig.abi.as_ref().map(|abi| {
            syn::Error::new_spanned(
                abi,
                format!(
                    "a component can't have an ABI, because only `html!` calls it, as a Rust \
                     function: remove `{}`",
                    abi.to_token_stream()
                ),
            )
        }),
        sig.receiver().map(|receiver| {
            syn::Error::new_spanned(
                receiver,
                "a component can't take `self`, because it's a function and not a method: \
                 move it out of the `impl` block and pass what it needs from `self` as a prop",
            )
        }),
    ];
    let combined = errors.into_iter().flatten().reduce(|mut all, error| {
        all.combine(error);
        all
    });
    match combined {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

/// The type the component is declared to return. Whether it is `HtmlFragment` is
/// left to the compiler (see [call_spanned_as]): the macro only sees how the type is
/// written, and an alias is as good. What it can see is that there's none at all,
/// or that it's written as a `Result` or an `Option`.
fn return_type(sig: &syn::Signature) -> syn::Result<&syn::Type> {
    let ty = match &sig.output {
        syn::ReturnType::Type(_, ty) => &**ty,
        syn::ReturnType::Default => {
            return Err(syn::Error::new(
                sig.ident.span(),
                "a component has to return an `HtmlFragment`: add `-> HtmlFragment`",
            ));
        }
    };

    // The compiler would refuse these two as well, but with a suggestion to add
    // `.expect("REASON")`, which ends up in the signature. Here the message can say
    // what to do. They're found by name, so not behind an alias: that's fine in this
    // direction, since nobody names a type that is an `HtmlFragment` `Result`.
    let advice = match last_path_segment(ty) {
        Some(name) if name == "Result" => {
            "a component can't return a `Result`, because `html!` has nowhere to pass an \
             error on to: do what can fail before `html!` and pass the result as a prop, \
             or return a fragment that shows the error"
        }
        Some(name) if name == "Option" => {
            "a component can't return an `Option`, because it always renders a fragment: \
             return `html! {}`, an empty one, where there's nothing to show"
        }
        _ => return Ok(ty),
    };
    Err(syn::Error::new_spanned(ty, advice))
}

/// The last name of a type written as a path: `Result` for `std::io::Result<T>`.
/// A type that comes from a `macro_rules!` argument (`$ty`) is wrapped in an
/// invisible group, and one in parentheses in a visible one: both are looked into.
fn last_path_segment(ty: &syn::Type) -> Option<&syn::Ident> {
    match ty {
        syn::Type::Path(path) => path.path.segments.last().map(|segment| &segment.ident),
        syn::Type::Group(group) => last_path_segment(&group.elem),
        syn::Type::Paren(paren) => last_path_segment(&paren.elem),
        _ => None,
    }
}

/// `function(args...)`, placed where `ty` is written in the user's code.
///
/// It's the last expression of the wrapper, which returns an `HtmlFragment`. If the
/// component returns something else, the compiler reports "mismatched types" for
/// this expression, and an expression is where its tokens say they are: from the
/// start of its first token to the end of its last. So the first one gets the span
/// of the type's first token and the last one that of its last, and the error
/// underlines the whole return type instead of `#[component]`.
fn call_spanned_as(
    ty: &syn::Type,
    function: &syn::Ident,
    args: &[&syn::Ident],
) -> proc_macro2::TokenStream {
    let mut type_spans = ty.to_token_stream().into_iter().map(|token| token.span());
    let (Some(first), last) = (type_spans.next(), type_spans.last()) else {
        // A type is never zero tokens
        return quote! { #function(#(#args),*) };
    };

    let mut function = function.clone();
    function.set_span(first);
    let mut args =
        proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, quote! { #(#args),* });
    args.set_span(last.unwrap_or(first));
    quote! { #function #args }
}

/// The content between a component's tags is passed as its `children` prop.
fn is_children(name: &syn::Ident) -> bool {
    name == "children"
}

/// `children` receives the content between the tags, so it must be a fragment.
/// `_children`, which rustc suggests for an unused parameter, wouldn't receive it.
fn check_children_prop(params: &[&syn::PatType], names: &[&syn::Ident]) -> syn::Result<()> {
    for (param, name) in params.iter().zip(names) {
        if *name == "_children" {
            return Err(syn::Error::new(
                name.span(),
                "the content between the tags goes to the prop named `children`, not `_children`; \
                 if the component doesn't use it, keep the name and add `#[allow(unused_variables)]`",
            ));
        }
        if is_children(name) && !is_html_fragment(&param.ty) {
            return Err(syn::Error::new_spanned(
                &param.ty,
                "`children` receives the content between the component's tags, \
                 so it must be an `HtmlFragment`",
            ));
        }
    }
    Ok(())
}

fn is_html_fragment(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "HtmlFragment"),
        _ => false,
    }
}

/// The name a prop parameter binds. `mut`, `ref` and `name @ pattern` still
/// bind one name, so they're allowed; other patterns have no name to use.
///
/// One name is taken: the props builder has a method for every prop, and one more
/// that `html!` ends with.
fn prop_name(pat: &syn::Pat) -> syn::Result<&syn::Ident> {
    match pat {
        syn::Pat::Ident(pat_ident) if pat_ident.ident.unraw() == BUILD_METHOD => {
            Err(syn::Error::new(
                pat_ident.ident.span(),
                format!(
                    "`{BUILD_METHOD}` can't be a prop name: `html!` calls a method with this \
                     name to finish a component's props"
                ),
            ))
        }
        syn::Pat::Ident(pat_ident) => Ok(&pat_ident.ident),
        _ => Err(syn::Error::new_spanned(
            pat,
            "component props need a name: use `name: Type` and destructure it in the function body",
        )),
    }
}

/// The first `impl Trait` in `ty`, at any depth (`impl Display`, `Vec<impl Display>`).
fn find_impl_trait(ty: &syn::Type) -> Option<&syn::TypeImplTrait> {
    struct ImplTraitFinder<'ast> {
        found: Option<&'ast syn::TypeImplTrait>,
    }

    impl<'ast> Visit<'ast> for ImplTraitFinder<'ast> {
        fn visit_type_impl_trait(&mut self, impl_trait: &'ast syn::TypeImplTrait) {
            self.found.get_or_insert(impl_trait);
        }
    }

    let mut finder = ImplTraitFinder { found: None };
    finder.visit_type(ty);
    finder.found
}

/// Names the lifetimes elided in the parameters (`&T`, `&mut T`, `'_`), declaring
/// the names on the function, so the parameter types can be Props struct fields.
///
/// Each one gets its own name, which is what elision means in parameters. One name
/// for all would tie them together: `&mut Vec<&str>` would become `&'a mut Vec<&'a str>`,
/// a borrow that has to last as long as the strings do.
///
/// A new name must be one the function doesn't write anywhere. The declared names
/// are in scope in the whole function, the body too, and `for<'a>` declares one
/// wherever it stands, which fails if `'a` is in scope already.
fn name_elided_lifetimes(function: &mut ItemFn) {
    let mut taken = HashSet::new();
    collect_lifetime_names(function.to_token_stream(), &mut taken);

    let sig = &mut function.sig;
    let mut namer = ElidedLifetimeNamer {
        taken,
        named: Vec::new(),
    };
    for input in &mut sig.inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            namer.visit_type_mut(&mut pat_type.ty);
        }
    }
    // Lifetimes go before type and const parameters. syn reorders them when
    // printing anyway, but the syntax tree shouldn't depend on that.
    for (index, lifetime) in namer.named.into_iter().enumerate() {
        let param = syn::GenericParam::Lifetime(syn::LifetimeParam::new(lifetime));
        sig.generics.params.insert(index, param);
    }
}

/// Adds every lifetime name written in `tokens` to `names`: declared or used, at any
/// depth, in the arguments of a macro too, which a syntax tree keeps as plain tokens.
fn collect_lifetime_names(tokens: proc_macro2::TokenStream, names: &mut HashSet<String>) {
    let mut tokens = tokens.into_iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Group(group) => collect_lifetime_names(group.stream(), names),
            // A lifetime is two tokens: the `'` and the name. A character like `'a'`
            // is a single literal token, so it doesn't get here.
            TokenTree::Punct(punct) if punct.as_char() == '\'' => {
                if let Some(TokenTree::Ident(name)) = tokens.peek() {
                    // `'r#a` is the same lifetime as `'a`
                    names.insert(format!("'{}", name.unraw()));
                }
            }
            _ => {}
        }
    }
}

struct ElidedLifetimeNamer {
    /// The names the function writes itself
    taken: HashSet<String>,
    /// The names given so far, in the order of the parameters
    named: Vec<syn::Lifetime>,
}

impl ElidedLifetimeNamer {
    /// A name nothing else uses: `'a`, `'a1`, `'a2`, ...
    fn fresh(&mut self) -> syn::Lifetime {
        let is_free = |name: &String| {
            !self.taken.contains(name) && !self.named.iter().any(|named| named.to_string() == *name)
        };
        let name = std::iter::once("'a".to_owned())
            .chain((1..).map(|suffix| format!("'a{suffix}")))
            .find(is_free)
            .expect("the iterator of names never ends");
        let lifetime = syn::Lifetime::new(&name, proc_macro2::Span::call_site());
        self.named.push(lifetime.clone());
        lifetime
    }
}

impl VisitMut for ElidedLifetimeNamer {
    fn visit_type_reference_mut(&mut self, reference: &mut syn::TypeReference) {
        if reference.lifetime.is_none() {
            reference.lifetime = Some(self.fresh());
        }
        syn::visit_mut::visit_type_reference_mut(self, reference);
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if lifetime.ident == "_" {
            *lifetime = self.fresh();
        }
    }

    // `fn(&T)` and `Fn(&T)` have their own elision scope: those references
    // borrow from the call's arguments, not from the props.
    fn visit_type_bare_fn_mut(&mut self, _: &mut syn::TypeBareFn) {}

    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        _: &mut syn::ParenthesizedGenericArguments,
    ) {
    }
}

fn is_builder_attr(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("builder")
}
