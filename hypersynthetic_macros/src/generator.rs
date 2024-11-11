use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{
    attributes::{AttrName, AttrValue, InterpolatedSegment, RegularAttribute},
    nodes::{Node, NodeCollection},
};

pub fn generate_nodes(NodeCollection::Nodes(nodes): NodeCollection) -> TokenStream2 {
    let nodes: Vec<TokenStream2> = nodes.into_iter().map(generate_node).collect();

    let nodes: Vec<TokenStream2> = nodes
        .into_iter()
        .map(|node| {
            quote! {
                v.extend(#node);
            }
        })
        .collect();

    quote! {
        {
            hypersynthetic::HtmlFragment::new({
                let mut v = vec![];
                #(#nodes)*
                v
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
                quote! {
                    {
                        let mut for_v = Vec::new();
                        for #var in #collection {
                            for_v.push(hypersynthetic::Node::Element(hypersynthetic::ElementData {
                                tag_name: #tag_name.to_owned(),
                                attributes: vec![#(#attributes),*],
                                children: #children,
                                self_closing: #self_closing,
                            }));
                        }
                        for_v
                    }
                }
            } else {
                quote! {
                    vec![hypersynthetic::Node::Element(hypersynthetic::ElementData {
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
            quote! {
                vec![hypersynthetic::Node::Text(hypersynthetic::escape_text(format!(#text)).to_string())]
            }
        }
        Node::Expression(expr) => {
            quote! {
                vec![hypersynthetic::Node::Text(hypersynthetic::escape_text(format!("{}", #expr)).to_string())]
            }
        }
        Node::UnescapedExpression(expr) => {
            quote! {
                vec![hypersynthetic::Node::Text(format!("{}", #expr))]
            }
        }
        Node::DocType => {
            quote! {
                vec![hypersynthetic::Node::DocType]
            }
        }
        Node::Component(component) => {
            let component_name = &component.name;
            let props: Vec<TokenStream2> = component
                .get_regular_attributes()
                .into_iter()
                .map(generate_attribute_as_prop)
                .collect();
            let children: TokenStream2 =
                generate_nodes(NodeCollection::Nodes(component.children.clone()));
            let has_slots = !component.children.is_empty();
            let component_call = if has_slots {
                quote! {
                    #component_name(#children, #(#props),*)
                }
            } else {
                quote! {
                    #component_name(#(#props),*)
                }
            };
            let tokens = if component.has_for_attribute() {
                let for_expr = component.get_for_attribute();
                let var = for_expr.pat;
                let collection = for_expr.collection;
                quote! {
                    {
                        let mut for_v = Vec::new();
                        for #var in #collection {
                            for_v.extend(#component_call.get_nodes());
                        }
                        for_v
                    }
                }
            } else {
                quote! {
                    #component_call.get_nodes()
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
        AttrName::Expression(expr) => {
            quote! { hypersynthetic::escape_attribute(format!("{}", #expr)).to_string() }
        }
    };

    let attr_value = match &attr.value {
        Some(AttrValue::Literal(value)) => {
            quote! { Some(hypersynthetic::escape_attribute(#value).to_string()) }
        }
        Some(AttrValue::Expression(expr)) => {
            quote! { Some(hypersynthetic::escape_attribute(format!("{}", #expr)).to_string()) }
        }
        Some(AttrValue::Interpolated(segments)) => interpolate_attr_value(segments),
        None => quote! { None },
    };

    quote! {
        hypersynthetic::Attribute {
            name: #attr_name,
            value: #attr_value,
        }
    }
}

fn generate_attribute_as_prop(attr: RegularAttribute) -> TokenStream2 {
    match &attr.value {
        Some(AttrValue::Literal(value)) => {
            quote! { #value }
        }
        Some(AttrValue::Expression(expr)) => {
            quote! { #expr }
        }
        Some(AttrValue::Interpolated(segments)) => interpolate_attr_value(segments),
        None => {
            quote! {}
        }
    }
}

fn interpolate_attr_value(segments: &[InterpolatedSegment]) -> TokenStream2 {
    let interpolated: Vec<TokenStream2> = segments
        .iter()
        .map(|segment| match segment {
            InterpolatedSegment::Str(s) => quote! { #s },
            InterpolatedSegment::Expr(e) => quote! { format!("{}", #e) },
            // InterpolatedSegment::Expr(e) => quote! { #e },
        })
        .collect();
    let format_pattern = generate_format_string_pattern(interpolated.len());
    let format_call = quote! { format!(#format_pattern, #(#interpolated),*) };
    quote! { Some(hypersynthetic::escape_attribute(#format_call).to_string()) }
}

fn generate_format_string_pattern(count: usize) -> TokenStream2 {
    let patterns: Vec<TokenStream2> = (0..count).map(|_| quote! {"{}"}).collect();
    let pattern_string = quote! { concat!(#(#patterns),*) };
    pattern_string
}
