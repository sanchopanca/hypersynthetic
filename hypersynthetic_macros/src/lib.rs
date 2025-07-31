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
use syn::{parse_macro_input, ItemFn};
use utils::is_pascal_case;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let parsed_html_nodes = parse_macro_input!(input as NodeCollection);
    let expanded = generate_nodes(parsed_html_nodes);
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input TokenStream into a syn::ItemFn
    let mut function: ItemFn = syn::parse(item.clone()).unwrap();

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

    // Add lifetime annotations to the original function if needed
    if function.sig.generics.lifetimes().count() == 0 {
        // Check if any parameter has a reference
        let has_refs = function.sig.inputs.iter().any(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                matches!(&*pat_type.ty, syn::Type::Reference(_))
            } else {
                false
            }
        });

        if has_refs {
            // Add a lifetime parameter to the original function
            let lifetime: syn::Lifetime = syn::parse_quote!('a);
            let lifetime_param = syn::GenericParam::Lifetime(syn::LifetimeParam {
                attrs: vec![],
                lifetime: lifetime.clone(),
                colon_token: None,
                bounds: syn::punctuated::Punctuated::new(),
            });
            function.sig.generics.params.push(lifetime_param);

            // Update reference types to use the lifetime
            for input in &mut function.sig.inputs {
                if let syn::FnArg::Typed(pat_type) = input {
                    if let syn::Type::Reference(type_ref) = &mut *pat_type.ty {
                        if type_ref.lifetime.is_none() {
                            type_ref.lifetime = Some(lifetime.clone());
                        }
                    }
                }
            }
        }
    }

    // Generate Props struct name
    let props_name = quote::format_ident!("{}Props", fn_name);
    let props_builder_name = quote::format_ident!("{}PropsBuilder", fn_name);

    // Check if the first parameter is HtmlFragment (slot)
    let has_slot = function.sig.inputs.first().is_some_and(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Type::Path(type_path) = &*pat_type.ty {
                type_path
                    .path
                    .segments
                    .last()
                    .is_some_and(|seg| seg.ident == "HtmlFragment")
            } else {
                false
            }
        } else {
            false
        }
    });

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

    // Generate struct fields
    let struct_fields = params.iter().map(|param| {
        let pat = &param.pat;
        let ty = &param.ty;
        quote! {
            #pat: #ty
        }
    });

    // Generate the internal function name
    let internal_fn_name = quote::format_ident!("__{}", fn_name);

    // Clone the original function and rename it
    let mut internal_function = function.clone();
    internal_function.sig.ident = internal_fn_name.clone();
    internal_function.vis = syn::Visibility::Inherited;

    // Add allow directive for snake_case to the internal function
    let allow_attr: syn::Attribute = syn::parse_quote!(#[allow(non_snake_case)]);
    internal_function.attrs.push(allow_attr);

    // Extract lifetimes and generics from the updated internal function
    let generics = &internal_function.sig.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate the parameter unpacking
    let param_names: Vec<_> = params.iter().map(|param| &param.pat).collect();

    // Generate the wrapper function
    let wrapper_fn = if has_slot {
        // Extract slot parameter name
        let slot_param = &function.sig.inputs[0];
        let slot_param_name = if let syn::FnArg::Typed(pat_type) = slot_param {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                &ident.ident
            } else {
                panic!("Slot parameter must be a simple identifier")
            }
        } else {
            panic!("Slot parameter must be a typed parameter")
        };

        quote! {
            #[allow(non_snake_case)]
            #vis fn #fn_name #impl_generics(#slot_param, props: #props_name #ty_generics) -> hypersynthetic::HtmlFragment #where_clause {
                let #props_name { #(#param_names),* } = props;
                #internal_fn_name(#slot_param_name, #(#param_names),*)
            }
        }
    } else {
        quote! {
            #[allow(non_snake_case)]
            #vis fn #fn_name #impl_generics(props: #props_name #ty_generics) -> hypersynthetic::HtmlFragment #where_clause {
                let #props_name { #(#param_names),* } = props;
                #internal_fn_name(#(#param_names),*)
            }
        }
    };

    // Generate the final output
    let output = quote! {
        #[derive(::hypersynthetic::typed_builder_macro::TypedBuilder)]
        #vis struct #props_name #generics #where_clause {
            #(#struct_fields,)*
        }

        impl #impl_generics hypersynthetic::component::Props for #props_name #ty_generics #where_clause {
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
