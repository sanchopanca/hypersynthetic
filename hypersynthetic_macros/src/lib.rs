extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, LitStr, Result, Token};
use syn::{parse_macro_input, Ident};

enum Node {
    Element(Tag),
    Text(LitStr),
}

struct Tag {
    tag_name: Ident,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
    self_closing: bool,
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

            let mut attributes = Vec::new();

            // found closing angle bracket
            while !input.peek(Token![>]) && !input.peek2(Token![>]) {
                let attribute: Attribute = input.parse()?;
                attributes.push(attribute);
            }

            // self-closing tag
            if input.peek(Token![/]) && input.peek2(Token![>]) {
                let _: Token![/] = input.parse()?;
                let _: Token![>] = input.parse()?;
                return Ok(Node::Element(Tag {
                    tag_name,
                    attributes,
                    children: Vec::new(),
                    self_closing: true,
                }));
            }
            let _: Token![>] = input.parse()?;

            let mut children: Vec<Node> = Vec::new();
            while input.peek(Token![<]) && input.peek2(Ident) || input.peek(LitStr) {
                let child: Node = input.parse()?;
                children.push(child);
            }

            let element = Tag {
                tag_name: tag_name.clone(),
                attributes,
                children,
                self_closing: false,
            };

            // this is a closing tag
            if input.peek(Token![<]) && input.peek2(Token![/]) && input.peek3(Ident) {
                let _: Token![<] = input.parse()?;
                let _: Token![/] = input.parse()?;
                let closing_tag_name: Ident = input.parse()?;
                if closing_tag_name != tag_name {
                    Err(input.error(format!(
                        "Expected closing tag {}, found {}",
                        tag_name, closing_tag_name
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

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value: LitStr = input.parse()?;
        Ok(Attribute { name, value })
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
            let self_closing = element.self_closing;
            let children: Vec<TokenStream2> =
                element.children.into_iter().map(generate_node).collect();
            let attributes: Vec<TokenStream2> = element
                .attributes
                .into_iter()
                .map(generate_attribute)
                .collect();
            quote! {
                hypersynthetic::Node::Element(hypersynthetic::ElementData {
                    tag_name: #tag_name.to_owned(),
                    attributes: vec![#(#attributes),*],
                    children: vec![#(#children),*],
                    self_closing: #self_closing,
                })
            }
        }
        Node::Text(text) => {
            quote! {
                hypersynthetic::Node::Text(#text.to_owned())
            }
        }
    }
}

fn generate_attribute(attr: Attribute) -> TokenStream2 {
    let name = attr.name.to_string();
    let value = attr.value;
    quote! {
        hypersynthetic::Attribute {
            name: #name.to_owned(),
            value: #value.to_owned(),
        }
    }
}
