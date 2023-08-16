extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, LitStr, Result, Token};
use syn::{parse_macro_input, Ident};

enum Node {
    Element(ElementData),
    Text(LitStr),
}

struct ElementData {
    tag_name: Ident,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
}

struct Attribute {
    name: Ident,
    value: LitStr,
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) && input.peek2(Ident) {
            let _: Token![<] = input.parse()?;
            let tag_name: Ident = input.parse()?;
            let _: Token![>] = input.parse()?;

            let element = ElementData {
                tag_name: tag_name.clone(),
                attributes: Vec::new(),
                children: vec![input.parse()?],
            };

            if input.peek(Token![<]) && input.peek2(Token![/]) && input.peek3(Ident) {
                let _: Token![<] = input.parse()?;
                let _: Token![/] = input.parse()?;
                let closing_tag_name: Ident = input.parse()?;
                if closing_tag_name != tag_name {
                    Err(input.error(format!(
                        "Expected closing tag {}, found {}",
                        closing_tag_name, tag_name
                    )))
                } else {
                    let _: Token![>] = input.parse()?;
                    Ok(Node::Element(element))
                }
            } else {
                Ok(Node::Text(input.parse()?))
            }
        } else if input.peek(LitStr) {
            let content: LitStr = input.parse()?;
            Ok(Node::Text(content))
        } else {
            Err(input.error("Expected a node"))
        }
    }
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let parsed_html = parse_macro_input!(input as Node);
    let expanded = generate_node(parsed_html);
    TokenStream::from(expanded)
}

fn generate_node(tag: Node) -> TokenStream2 {
    match tag {
        Node::Element(element) => {
            let tag_name = element.tag_name.to_string();
            let children: Vec<TokenStream2> =
                element.children.into_iter().map(generate_node).collect();
            quote! {
                hypersynthetic::Node::Element(hypersynthetic::ElementData {
                    tag_name: #tag_name.to_owned(),
                    attributes: vec![],
                    children: vec![#(#children),*]
                })
            }
        }
        Node::Text(text) => {
            quote! {
                hypersynthetic::Node::Text(#text.to_string())
            }
        }
    }
}
