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
    Iterator(Expr),
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

            if let Expr::MethodCall(method) = &content_expr {
                if method.method == "map" {
                    return Ok(Node::Iterator(content_expr));
                }
            }
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
            hypersynthetic::NodeCollection::new({
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
            let children: TokenStream2 = generate_nodes(NodeCollection::Nodes(element.children));
            let attributes: Vec<TokenStream2> = element
                .attributes
                .into_iter()
                .map(generate_attribute)
                .collect();
            quote! {
                vec![hypersynthetic::Node::Element(hypersynthetic::ElementData {
                    tag_name: #tag_name.to_owned(),
                    attributes: vec![#(#attributes),*],
                    children: #children,
                    self_closing: #self_closing,
                })]
            }
        }
        Node::Text(text) => {
            quote! {
                vec![hypersynthetic::Node::Text(hypersynthetic::escape_text(#text).to_string())]
            }
        }
        Node::Expression(expr) => {
            quote! {
                vec![hypersynthetic::Node::Text(hypersynthetic::escape_text(format!("{}", #expr)).to_string())]
            }
        }
        Node::Iterator(expr) => {
            quote! {
                #expr.collect::<Vec<_>>().into_iter().flat_map(|collection| collection.get_nodes())
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
                .props
                .into_iter()
                .map(generate_attribute_as_prop)
                .collect();
            quote! {
                #component_name(#(#props),*).get_nodes()
            }
        }
    }
}

fn generate_attribute(attr: Attribute) -> TokenStream2 {
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
        None => quote! { None },
    };

    quote! {
        hypersynthetic::Attribute {
            name: #attr_name,
            value: #attr_value,
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
