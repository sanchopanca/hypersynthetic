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
use syn::{ItemFn, parse_macro_input};
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

    // Helper function to check if a type contains any references
    fn type_contains_refs(ty: &syn::Type) -> bool {
        match ty {
            syn::Type::Reference(_) => true,
            syn::Type::Path(type_path) => {
                // Check generic arguments
                type_path.path.segments.iter().any(|segment| {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        args.args.iter().any(|arg| {
                            if let syn::GenericArgument::Type(inner_ty) = arg {
                                type_contains_refs(inner_ty)
                            } else {
                                false
                            }
                        })
                    } else {
                        false
                    }
                })
            }
            syn::Type::Tuple(type_tuple) => type_tuple.elems.iter().any(type_contains_refs),
            syn::Type::Array(type_array) => type_contains_refs(&type_array.elem),
            syn::Type::Slice(type_slice) => type_contains_refs(&type_slice.elem),
            syn::Type::Paren(type_paren) => type_contains_refs(&type_paren.elem),
            syn::Type::Group(type_group) => type_contains_refs(&type_group.elem),
            _ => false,
        }
    }

    // Helper function to add lifetime to references in a type
    fn add_lifetime_to_refs(ty: &mut syn::Type, lifetime: &syn::Lifetime) {
        match ty {
            syn::Type::Reference(type_ref) => {
                if type_ref.lifetime.is_none() {
                    type_ref.lifetime = Some(lifetime.clone());
                }
                // Also process the inner type to handle nested references
                add_lifetime_to_refs(&mut type_ref.elem, lifetime);
            }
            syn::Type::Path(type_path) => {
                // Add lifetime to generic arguments
                for segment in &mut type_path.path.segments {
                    if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                        for arg in &mut args.args {
                            if let syn::GenericArgument::Type(inner_ty) = arg {
                                add_lifetime_to_refs(inner_ty, lifetime);
                            }
                        }
                    }
                }
            }
            syn::Type::Tuple(type_tuple) => {
                for elem in &mut type_tuple.elems {
                    add_lifetime_to_refs(elem, lifetime);
                }
            }
            syn::Type::Array(type_array) => add_lifetime_to_refs(&mut type_array.elem, lifetime),
            syn::Type::Slice(type_slice) => add_lifetime_to_refs(&mut type_slice.elem, lifetime),
            syn::Type::Paren(type_paren) => add_lifetime_to_refs(&mut type_paren.elem, lifetime),
            syn::Type::Group(type_group) => add_lifetime_to_refs(&mut type_group.elem, lifetime),
            _ => {}
        }
    }

    // Add lifetime annotations to the original function if needed
    if function.sig.generics.lifetimes().count() == 0 {
        // Check if any parameter has a reference
        let has_refs = function.sig.inputs.iter().any(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                type_contains_refs(&pat_type.ty)
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
                    add_lifetime_to_refs(&mut pat_type.ty, &lifetime);
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

    // Check if this is a no-parameter component (excluding slots)
    let is_no_params = params.is_empty();

    // Generate wrapper functions
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

        if is_no_params {
            // For slot components with no parameters, generate both functions
            let props_fn_name = quote::format_ident!("__{}_with_props", fn_name);
            quote! {
                // Direct callable function for slots (takes just the HtmlFragment)
                #[allow(non_snake_case)]
                #vis fn #fn_name #impl_generics(#slot_param) -> hypersynthetic::HtmlFragment #where_clause {
                    #internal_fn_name(#slot_param_name)
                }

                // Props-based function for JSX usage (with different name)
                #[allow(non_snake_case)]
                #vis fn #props_fn_name #impl_generics(#slot_param, props: #props_name #ty_generics) -> hypersynthetic::HtmlFragment #where_clause {
                    let #props_name { #(#param_names),* } = props;
                    #internal_fn_name(#slot_param_name, #(#param_names),*)
                }
            }
        } else {
            quote! {
                #[allow(non_snake_case)]
                #vis fn #fn_name #impl_generics(#slot_param, props: #props_name #ty_generics) -> hypersynthetic::HtmlFragment #where_clause {
                    let #props_name { #(#param_names),* } = props;
                    #internal_fn_name(#slot_param_name, #(#param_names),*)
                }
            }
        }
    } else if is_no_params {
        // For no-parameter components, generate both functions
        let props_fn_name = quote::format_ident!("__{}_with_props", fn_name);
        quote! {
            // Direct callable function (no arguments)
            #[allow(non_snake_case)]
            #vis fn #fn_name #impl_generics() -> hypersynthetic::HtmlFragment #where_clause {
                #internal_fn_name()
            }

            // Props-based function for JSX usage (with different name)
            #[allow(non_snake_case)]
            #vis fn #props_fn_name #impl_generics(props: #props_name #ty_generics) -> hypersynthetic::HtmlFragment #where_clause {
                let #props_name { #(#param_names),* } = props;
                #internal_fn_name(#(#param_names),*)
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

    // Generate the final output - always generate Props struct
    let output = quote! {
        #[derive(::hypersynthetic::typed_builder_macro::TypedBuilder)]
        #vis struct #props_name #impl_generics #where_clause {
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
