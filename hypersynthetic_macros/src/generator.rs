use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{LitStr, spanned::Spanned};

use crate::{
    attributes::{AttrName, AttrValue, InterpolatedSegment, InterpolatedString, RegularAttribute},
    nodes::{Node, NodeCollection},
};

/// Identifier for a variable declared by the generated code.
///
/// `Span::mixed_site()` gives local variables `macro_rules!`-style hygiene: they are
/// invisible to user code passed into the macro, so they can't shadow user variables.
fn internal_ident(name: &str) -> Ident {
    Ident::new(name, Span::mixed_site())
}

pub fn generate_nodes(NodeCollection::Nodes(nodes): NodeCollection) -> TokenStream2 {
    let v = internal_ident("__hs_nodes");
    let nodes: Vec<TokenStream2> = nodes.into_iter().map(generate_node).collect();

    let nodes: Vec<TokenStream2> = nodes
        .into_iter()
        .map(|node| {
            quote! {
                #v.extend(#node);
            }
        })
        .collect();

    quote! {
        {
            ::hypersynthetic::HtmlFragment::new({
                let mut #v = vec![];
                #(#nodes)*
                #v
            })
        }
    }
}

fn generate_node(tag: Node) -> TokenStream2 {
    match tag {
        Node::Element(element) => {
            let tag_name = element.tag_name.to_string();
            let self_closing = element.self_closing;
            let children: TokenStream2 =
                generate_nodes(NodeCollection::Nodes(element.children.clone()));
            let attributes: Vec<TokenStream2> = element
                .get_regular_attributes()
                .into_iter()
                .map(generate_attribute)
                .collect();
            let tokens = if element.has_for_attribute() {
                let for_expr = element.get_for_attribute();
                let var = for_expr.pat;
                let collection = for_expr.collection;
                let for_v = internal_ident("__hs_for_nodes");
                quote! {
                    {
                        let mut #for_v = Vec::new();
                        for #var in #collection {
                            #for_v.push(::hypersynthetic::Node::Element(::hypersynthetic::ElementData {
                                tag_name: #tag_name.to_owned(),
                                attributes: vec![#(#attributes),*],
                                children: #children,
                                self_closing: #self_closing,
                            }));
                        }
                        #for_v
                    }
                }
            } else {
                quote! {
                    vec![::hypersynthetic::Node::Element(::hypersynthetic::ElementData {
                        tag_name: #tag_name.to_owned(),
                        attributes: vec![#(#attributes),*],
                        children: #children,
                        self_closing: #self_closing,
                    })]
                }
            };

            if element.has_if_attribute() {
                let if_expr = element.get_if_attribute();
                quote! {
                    if #if_expr {
                        #tokens
                    } else {
                        vec![]
                    }
                }
            } else {
                tokens
            }
        }
        Node::Text(text) => {
            let text = generate_format(&text);
            quote! {
                vec![::hypersynthetic::Node::Text(::hypersynthetic::escape_text(#text).to_string())]
            }
        }
        Node::Expression(expr) => {
            quote! {
                vec![::hypersynthetic::Node::Text(::hypersynthetic::escape_text(format!("{}", #expr)).to_string())]
            }
        }
        Node::UnescapedExpression(expr) => {
            quote! {
                vec![::hypersynthetic::Node::Text(format!("{}", #expr))]
            }
        }
        Node::DocType => {
            quote! {
                vec![::hypersynthetic::Node::DocType]
            }
        }
        Node::Component(component) => {
            let component_name = &component.name;
            let attributes = component.get_regular_attributes();

            // Generate builder method calls
            let builder_calls: Vec<TokenStream2> = attributes
                .iter()
                .map(|attr| {
                    // Extract the attribute name
                    let attr_name = match &attr.name {
                        AttrName::Literal(name) => {
                            let name_str = name.value();
                            quote::format_ident!("{}", name_str, span = name.span())
                        }
                        AttrName::Expression(_) => {
                            unreachable!("rejected by the parser in validate_prop_names")
                        }
                    };

                    // Extract the attribute value
                    let attr_value = match &attr.value {
                        Some(AttrValue::Literal(value)) => quote! { #value },
                        Some(AttrValue::Expression(expr)) => quote! { #expr },
                        Some(AttrValue::Interpolated(string)) => generate_format(string),
                        // `<C disabled />` is short for `<C disabled={true} />`
                        None => quote::quote_spanned! { attr_name.span()=> true },
                    };

                    quote! { .#attr_name(#attr_value) }
                })
                .collect();

            // Children go to the slot, a Props field named `children`. Without
            // children it isn't set and defaults to an empty fragment. A component
            // without a slot has no such setter, which makes children an error.
            let children_call = if component.children.is_empty() {
                quote! {}
            } else {
                let children = generate_nodes(NodeCollection::Nodes(component.children.clone()));
                let setter = Ident::new("children", component_name.span());
                quote! { .#setter(#children) }
            };

            let final_call = quote! {
                ::hypersynthetic::component::component_view(
                    &#component_name,
                    ::hypersynthetic::component::component_props_builder(&#component_name)
                        #(#builder_calls)*
                        #children_call
                        .build()
                )
            };

            let tokens = if component.has_for_attribute() {
                let for_expr = component.get_for_attribute();
                let var = for_expr.pat;
                let collection = for_expr.collection;
                let for_v = internal_ident("__hs_for_nodes");
                quote! {
                    {
                        let mut #for_v = Vec::new();
                        for #var in #collection {
                            #for_v.extend(#final_call.get_nodes());
                        }
                        #for_v
                    }
                }
            } else {
                quote! {
                    #final_call.get_nodes()
                }
            };

            if component.has_if_attribute() {
                let if_expr = component.get_if_attribute();
                quote! {
                    if #if_expr {
                        #tokens
                    } else {
                        vec![]
                    }
                }
            } else {
                tokens
            }
        }
    }
}

fn generate_attribute(attr: RegularAttribute) -> TokenStream2 {
    let attr_name = match &attr.name {
        AttrName::Literal(name) => quote! { #name.to_owned() },
        // Not escaped: names are validated when rendering instead.
        AttrName::Expression(expr) => quote! { format!("{}", #expr) },
    };

    let attr_value = match &attr.value {
        Some(AttrValue::Literal(value)) => {
            quote! { Some(::hypersynthetic::escape_attribute(#value).to_string()) }
        }
        Some(AttrValue::Expression(expr)) => {
            quote! { Some(::hypersynthetic::escape_attribute(format!("{}", #expr)).to_string()) }
        }
        Some(AttrValue::Interpolated(string)) => {
            let value = generate_format(string);
            quote! { Some(::hypersynthetic::escape_attribute(#value).to_string()) }
        }
        None => quote! { None },
    };

    quote! {
        ::hypersynthetic::Attribute {
            name: #attr_name,
            value: #attr_value,
        }
    }
}

/// A `format!` call that produces the string's value.
fn generate_format(string: &InterpolatedString) -> TokenStream2 {
    // Passing the literal unchanged lets rustc point errors at the exact
    // `{name}` inside it, which it can't do for a string we build.
    if string.is_format_string {
        let lit = &string.lit;
        return quote! { format!(#lit) };
    }

    let mut format_string = String::new();
    let mut args = Vec::new();
    for segment in &string.segments {
        match segment {
            InterpolatedSegment::Str(text) => {
                format_string.push_str(&text.replace('{', "{{").replace('}', "}}"));
            }
            InterpolatedSegment::Expr { expr, spec } => {
                format_string.push('{');
                if let Some(spec) = spec {
                    format_string.push(':');
                    format_string.push_str(spec);
                }
                format_string.push('}');
                args.push(expr);
            }
        }
    }
    let format_string = LitStr::new(&format_string, string.lit.span());
    quote! { format!(#format_string, #(#args),*) }
}
