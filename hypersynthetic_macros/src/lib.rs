mod attributes;
mod generator;
mod nodes;
mod parser;
mod utils;

extern crate proc_macro;

use generator::generate_nodes;
use nodes::NodeCollection;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, visit::Visit, visit_mut::VisitMut};
use utils::is_pascal_case;

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

    // Props struct fields can't have elided lifetimes
    name_elided_lifetimes(&mut function.sig);

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

    // The first parameter is the slot if it's an HtmlFragment
    let slot_param = function.sig.inputs.first().and_then(|arg| match arg {
        syn::FnArg::Typed(pat_type) => match &*pat_type.ty {
            syn::Type::Path(type_path)
                if type_path
                    .path
                    .segments
                    .last()
                    .is_some_and(|seg| seg.ident == "HtmlFragment") =>
            {
                Some(pat_type)
            }
            _ => None,
        },
        syn::FnArg::Receiver(_) => None,
    });
    let has_slot = slot_param.is_some();

    // Extract parameters (skip first if it's a slot)
    let params: Vec<_> = function
        .sig
        .inputs
        .iter()
        .skip(if has_slot { 1 } else { 0 })
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

    // The slot is passed as the `children` prop, so no other prop can have that name
    let children_prop = param_names.iter().find(|name| **name == "children");
    if let (Some(_), Some(name)) = (slot_param, children_prop) {
        return syn::Error::new(
            name.span(),
            "a component with a slot can't have a prop named `children`, it's reserved for the slot",
        )
        .to_compile_error()
        .into();
    }

    // Generate struct fields, with the parameter's `#[builder(...)]` attributes
    // (`default`, `setter(into)`, ...) for the TypedBuilder derive
    let struct_fields = params.iter().zip(&param_names).map(|(param, name)| {
        let ty = &param.ty;
        let builder_attrs = param.attrs.iter().filter(|attr| is_builder_attr(attr));
        quote! {
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

    // The slot is a Props field named `children`, which html! sets when the
    // component has children. It defaults to an empty fragment, so a component
    // with a slot can be used without children. The wrapper binds it to its own
    // name, because the user's parameter may not be a name at all (`_`).
    let slot_var = syn::Ident::new("slot", proc_macro2::Span::mixed_site());
    let (slot_field, slot_binding, slot_arg) = match slot_param {
        Some(slot_param) => {
            let slot_ty = &slot_param.ty;
            (
                quote! {
                    #[builder(default)]
                    children: #slot_ty,
                },
                quote! { children: #slot_var, },
                quote! { #slot_var, },
            )
        }
        None => (quote! {}, quote! {}, quote! {}),
    };

    let wrapper_fn = quote! {
        #(#wrapper_attrs)*
        #[allow(non_snake_case)]
        #vis fn #fn_name #impl_generics(props: #props_name #ty_generics) -> ::hypersynthetic::HtmlFragment #where_clause {
            let #props_name { #slot_binding #(#param_names),* } = props;
            #internal_fn_name(#slot_arg #(#param_names),*)
        }
    };

    // Generate the final output - always generate Props struct
    let output = quote! {
        #[doc = #props_doc]
        #[derive(::hypersynthetic::__private::typed_builder::TypedBuilder)]
        // The generated builder code refers to typed_builder, which users don't
        // depend on directly
        #[builder(crate_module_path = ::hypersynthetic::__private::typed_builder)]
        #vis struct #props_name #impl_generics #where_clause {
            #slot_field
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

/// The name a prop parameter binds. `mut`, `ref` and `name @ pattern` still
/// bind one name, so they're allowed; other patterns have no name to use.
fn prop_name(pat: &syn::Pat) -> syn::Result<&syn::Ident> {
    match pat {
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
/// the name on the function, so the parameter types can be Props struct fields.
fn name_elided_lifetimes(sig: &mut syn::Signature) {
    let mut namer = ElidedLifetimeNamer {
        lifetime: fresh_lifetime(&sig.generics),
        found: false,
    };
    for input in &mut sig.inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            namer.visit_type_mut(&mut pat_type.ty);
        }
    }
    if namer.found {
        // Lifetimes go before type and const parameters. syn reorders them when
        // printing anyway, but the syntax tree shouldn't depend on that.
        let param = syn::LifetimeParam::new(namer.lifetime);
        sig.generics
            .params
            .insert(0, syn::GenericParam::Lifetime(param));
    }
}

/// `'a`, or `'a1`, `'a2`, ... if the function already declares it.
fn fresh_lifetime(generics: &syn::Generics) -> syn::Lifetime {
    let taken: Vec<String> = generics
        .lifetimes()
        .map(|param| param.lifetime.to_string())
        .collect();
    let mut name = "'a".to_owned();
    let mut suffix = 0;
    while taken.contains(&name) {
        suffix += 1;
        name = format!("'a{suffix}");
    }
    syn::Lifetime::new(&name, proc_macro2::Span::call_site())
}

struct ElidedLifetimeNamer {
    lifetime: syn::Lifetime,
    found: bool,
}

impl VisitMut for ElidedLifetimeNamer {
    fn visit_type_reference_mut(&mut self, reference: &mut syn::TypeReference) {
        if reference.lifetime.is_none() {
            reference.lifetime = Some(self.lifetime.clone());
            self.found = true;
        }
        syn::visit_mut::visit_type_reference_mut(self, reference);
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if lifetime.ident == "_" {
            *lifetime = self.lifetime.clone();
            self.found = true;
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
