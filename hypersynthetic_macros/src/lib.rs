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

    // Add the allow attribute to suppress the non_snake_case warning
    // We require function to be in PascalCase
    let allow_attr: syn::Attribute = syn::parse_quote!(#[allow(non_snake_case)]);
    function.attrs.push(allow_attr);

    quote!(#function).into()
}
