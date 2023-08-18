extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Brace,
    Expr, Ident, ItemFn, LitStr, Result, Token,
};

enum NodeCollection {
    Nodes(Vec<Node>),
}

enum Node {
    Component(Component),
    DocType,
    Element(Tag),
    Expression(Expr),
    Text(LitStr),
}

struct Tag {
    tag_name: Ident,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
    self_closing: bool,
}

struct Component {
    name: Ident,
    props: Vec<Attribute>,
}

struct Attribute {
    name: AttrName,
    value: Option<AttrValue>,
}

enum AttrName {
    Literal(LitStr),
    Expression(Expr),
}

enum AttrValue {
    Literal(LitStr),
    Expression(Expr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) && input.peek2(Token![!]) {
            let _: Token![<] = input.parse()?;
            let _: Token![!] = input.parse()?;

            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                let ident: Ident = input.parse()?;
                if ident.to_string().to_lowercase() == "doctype" {
                    let html_ident: Ident = input.parse()?;
                    if html_ident.to_string().to_lowercase() == "html" {
                        let _: Token![>] = input.parse()?;
                        return Ok(Node::DocType);
                    }
                }
            }
        }

        if input.peek(Token![<]) && input.peek2(Ident) {
            let _: Token![<] = input.parse()?;
            let tag_name: Ident = input.parse()?;

            let mut attributes = Vec::new();

            // found closing angle bracket
            while !input.peek(Token![>]) && !input.peek2(Token![>]) {
                let attribute: Attribute = input.parse()?;
                attributes.push(attribute);
            }

            // this is a component
            if is_pascal_case(&tag_name) {
                // components are always self-closing
                let _: Token![/] = input.parse()?;
                let _: Token![>] = input.parse()?;

                return Ok(Node::Component(Component {
                    name: tag_name,
                    props: attributes,
                }));
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
            while input.peek(Token![<]) && input.peek2(Ident)
                || input.peek(LitStr)
                || input.peek(Brace)
            {
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
        } else if input.peek(Brace) {
            let content_brackets;
            braced!(content_brackets in input);
            let content_expr: Expr = content_brackets.parse()?;
            Ok(Node::Expression(content_expr))
        } else {
            Err(input.error("Expected a node"))
        }
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: AttrName = input.parse()?;

        // If the next token is '=', then expect a value. Otherwise, no value.
        let value = if input.peek(Token![=]) {
            let _: Token![=] = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Attribute { name, value })
    }
}

impl Parse for AttrName {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Brace) {
            let content_brackets;
            braced!(content_brackets in input);
            let content_expr: Expr = content_brackets.parse()?;
            return Ok(AttrName::Expression(content_expr));
        }

        let span = input.span();

        let mut name = String::new();
        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                let ident: Ident = input.parse()?;
                name.push_str(&ident.to_string());
            } else if lookahead.peek(Token![type]) {
                // TODO all the rest of keywords
                let _: Token![type] = input.parse()?;
                name.push_str("type");
            } else if lookahead.peek(Token![-]) {
                let _: Token![-] = input.parse()?;
                name.push('-');
            } else {
                break;
            }
        }

        if !name.is_empty() {
            Ok(AttrName::Literal(LitStr::new(&name, span)))
        } else {
            Err(input.error("Expected a valid attribute name"))
        }
    }
}

impl Parse for AttrValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Brace) {
            let content_brackets;
            braced!(content_brackets in input);
            let content_expr: Expr = content_brackets.parse()?;
            Ok(AttrValue::Expression(content_expr))
        } else {
            let lit_str: LitStr = input.parse()?;
            Ok(AttrValue::Literal(lit_str))
        }
    }
}

impl Parse for NodeCollection {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            nodes.push(input.parse()?);
        }
        Ok(NodeCollection::Nodes(nodes))
    }
}

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

// This is one lazy implementation that only checks that the first character is uppercase
fn is_pascal_case(name: &Ident) -> bool {
    let first_char = name.to_string().chars().next();
    matches!(first_char, Some(ch) if ch.is_uppercase())
}

fn generate_nodes(NodeCollection::Nodes(nodes): NodeCollection) -> TokenStream2 {
    let nodes: Vec<TokenStream2> = nodes.into_iter().map(generate_node).collect();
    if nodes.len() == 1 {
        let node = nodes[0].clone();
        quote! {
            #node
        }
    } else {
        quote! {
            hypersynthetic_types::NodeCollection::new(vec![#(#nodes),*])
        }
    }
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
                hypersynthetic_types::Node::Element(hypersynthetic_types::ElementData {
                    tag_name: #tag_name.to_owned(),
                    attributes: vec![#(#attributes),*],
                    children: vec![#(#children),*],
                    self_closing: #self_closing,
                })
            }
        }
        Node::Text(text) => {
            quote! {
                hypersynthetic_types::Node::Text(#text.to_owned())
            }
        }
        Node::Expression(expr) => {
            quote! {
                hypersynthetic_types::Node::Text(format!("{}", #expr))
            }
        }
        Node::DocType => {
            quote! {
                hypersynthetic_types::Node::DocType
            }
        }
        Node::Component(component) => {
            let component_name = &component.name;
            let props: Vec<TokenStream2> = component
                .props
                .into_iter()
                .map(generate_attribute_as_prop) // Create this function. It will transform attributes into function arguments.
                .collect();
            quote! {
                #component_name(#(#props),*)
            }
        }
    }
}

fn generate_attribute(attr: Attribute) -> TokenStream2 {
    match &attr.name {
        AttrName::Literal(name) => {
            let name_literal = quote! { #name.to_owned() };

            match &attr.value {
                Some(AttrValue::Literal(value)) => {
                    quote! {
                        hypersynthetic_types::Attribute {
                            name: #name_literal,
                            value: Some(#value.to_owned()),
                        }
                    }
                }
                Some(AttrValue::Expression(expr)) => {
                    quote! {
                        hypersynthetic_types::Attribute {
                            name: #name_literal,
                            value: Some(format!("{}", #expr)),
                        }
                    }
                }
                None => {
                    quote! {
                        hypersynthetic_types::Attribute {
                            name: #name_literal,
                            value: None,
                        }
                    }
                }
            }
        }
        AttrName::Expression(name_expr) => {
            let name_expression = quote! { format!("{}", #name_expr) };

            match &attr.value {
                Some(AttrValue::Literal(value)) => {
                    quote! {
                        hypersynthetic_types::Attribute {
                            name: #name_expression,
                            value: Some(#value.to_owned()),
                        }
                    }
                }
                Some(AttrValue::Expression(value_expr)) => {
                    quote! {
                        hypersynthetic_types::Attribute {
                            name: #name_expression,
                            value: Some(format!("{}", #value_expr)),
                        }
                    }
                }
                None => {
                    quote! {
                        hypersynthetic_types::Attribute {
                            name: #name_expression,
                            value: None,
                        }
                    }
                }
            }
        }
    }
}

fn generate_attribute_as_prop(attr: Attribute) -> TokenStream2 {
    match &attr.value {
        Some(AttrValue::Literal(value)) => {
            quote! { #value }
        }
        Some(AttrValue::Expression(expr)) => {
            quote! { #expr }
        }
        None => {
            quote! {}
        }
    }
}
