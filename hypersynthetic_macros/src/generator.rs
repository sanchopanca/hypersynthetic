use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{LitStr, spanned::Spanned};

use crate::{
    attributes::{
        AttrName, AttrValue, ForExpr, InterpolatedSegment, InterpolatedString, RegularAttribute,
    },
    nodes::{Node, NodeCollection, for_attribute, if_attribute, regular_attributes},
};

/// Identifier for a variable declared by the generated code.
///
/// `Span::mixed_site()` gives local variables `macro_rules!`-style hygiene: they are
/// invisible to user code passed into the macro, so they can't shadow user variables.
fn internal_ident(name: &str) -> Ident {
    Ident::new(name, Span::mixed_site())
}

pub fn generate_nodes(NodeCollection::Nodes(nodes): NodeCollection) -> TokenStream2 {
    let target = internal_ident("__hs_nodes");
    let statements = nodes.into_iter().map(|node| generate_node(node, &target));

    quote! {
        {
            let mut #target = ::std::vec::Vec::new();
            #(#statements)*
            ::hypersynthetic::HtmlFragment::new(#target)
        }
    }
}

/// Statements that add the node to the `target` vector.
fn generate_node(node: Node, target: &Ident) -> TokenStream2 {
    match node {
        Node::Element(element) => {
            let tag_name = element.tag_name.to_string();
            let self_closing = element.self_closing;
            let children: TokenStream2 = generate_nodes(NodeCollection::Nodes(element.children));
            let attributes: Vec<TokenStream2> = regular_attributes(&element.attributes)
                .map(generate_attribute)
                .collect();
            let push = quote! {
                #target.push(::hypersynthetic::Node::Element(::hypersynthetic::ElementData {
                    tag_name: #tag_name.to_owned(),
                    attributes: vec![#(#attributes),*],
                    children: #children,
                    self_closing: #self_closing,
                }));
            };
            wrap_in_for_and_if(
                push,
                for_attribute(&element.attributes),
                if_attribute(&element.attributes),
            )
        }
        Node::Text(text) => {
            let text = generate_format(&text);
            quote! {
                #target.push(::hypersynthetic::Node::Text(::hypersynthetic::escape_text(#text).into_owned()));
            }
        }
        // See `hypersynthetic::__private` for how fragments and other values are told apart
        Node::Expression(expr) => render_expression(&expr, true, target),
        Node::UnescapedExpression(expr) => render_expression(&expr, false, target),
        Node::DocType => {
            quote! {
                #target.push(::hypersynthetic::Node::DocType);
            }
        }
        Node::Component(component) => {
            let component_name = &component.name;
            // Generate builder method calls
            let builder_calls: Vec<TokenStream2> = regular_attributes(&component.props)
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
                let children = generate_nodes(NodeCollection::Nodes(component.children));
                let setter = Ident::new("children", component_name.span());
                quote! { .#setter(#children) }
            };

            let extend = quote! {
                #target.extend(
                    ::hypersynthetic::component::component_view(
                        &#component_name,
                        ::hypersynthetic::component::component_props_builder(&#component_name)
                            #(#builder_calls)*
                            #children_call
                            .build()
                    )
                    .into_nodes()
                );
            };
            wrap_in_for_and_if(
                extend,
                for_attribute(&component.props),
                if_attribute(&component.props),
            )
        }
    }
}

/// Puts the statements in the `:for` loop, and that in the `:if` condition, so the
/// condition is checked once, before the loop.
fn wrap_in_for_and_if(
    statements: TokenStream2,
    for_attribute: Option<&ForExpr>,
    if_attribute: Option<&syn::Expr>,
) -> TokenStream2 {
    let statements = match for_attribute {
        Some(ForExpr { pat, collection }) => quote! {
            for #pat in #collection {
                #statements
            }
        },
        None => statements,
    };
    match if_attribute {
        Some(condition) => quote! {
            if #condition {
                #statements
            }
        },
        None => statements,
    }
}

fn generate_attribute(attr: &RegularAttribute) -> TokenStream2 {
    let attr_name = match &attr.name {
        AttrName::Literal(name) => quote! { #name.to_owned() },
        // Not escaped: names are validated when rendering instead.
        AttrName::Expression(expr) => quote! { format!("{}", #expr) },
    };

    // Values are stored unescaped and escaped when rendering
    let attr_value = match &attr.value {
        Some(AttrValue::Literal(value)) => quote! { Some(#value.to_owned()) },
        Some(AttrValue::Expression(expr)) => quote! { Some(format!("{}", #expr)) },
        Some(AttrValue::Interpolated(string)) => {
            let value = generate_format(string);
            quote! { Some(#value) }
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

fn render_expression(expr: &syn::Expr, escape: bool, target: &Ident) -> TokenStream2 {
    // Errors about the method (e.g. the value isn't Display) are reported at its
    // name, so give it the expression's span
    let render = Ident::new("render", expr.span());
    quote! {
        {
            #[allow(unused_imports)]
            use ::hypersynthetic::__private::{
                RenderDisplay as _, RenderFragment as _, RenderOption as _,
            };
            (&::hypersynthetic::__private::Render(&(#expr))).#render(#escape, &mut #target);
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
