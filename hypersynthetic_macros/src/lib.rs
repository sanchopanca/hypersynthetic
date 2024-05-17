extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens as _};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Brace,
    Expr, Ident, ItemFn, LitStr, Pat, Path, Result, Token,
};

#[derive(Clone)]
enum NodeCollection {
    Nodes(Vec<Node>),
}

#[derive(Clone)]
enum Node {
    Component(Component),
    DocType,
    Element(Tag),
    Expression(Expr),
    Text(LitStr),
    UnescapedExpression(Expr),
}

#[derive(Clone)]
struct Tag {
    tag_name: Ident,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
    self_closing: bool,
}

#[derive(Clone)]
struct Component {
    name: Path,
    props: Vec<Attribute>,
    children: Vec<Node>,
}

#[derive(Clone)]
enum Attribute {
    RegularAttribute(RegularAttribute),
    For(ForExpr),
}

#[derive(Clone)]
struct RegularAttribute {
    name: AttrName,
    value: Option<AttrValue>,
}

#[derive(Clone)]
enum AttrName {
    Literal(LitStr),
    Expression(Expr),
}

#[derive(Clone)]
enum AttrValue {
    Literal(LitStr),
    Expression(Expr),
    Interpolated(Vec<InterpolatedSegment>),
}

#[derive(Clone)]
enum InterpolatedSegment {
    Str(LitStr),
    Expr(Expr),
}

#[derive(Clone)]
struct ForExpr {
    pat: Pat,
    collection: Expr,
}

impl Tag {
    fn has_for_attribute(&self) -> bool {
        self.attributes
            .iter()
            .any(|attr| matches!(attr, Attribute::For(_)))
    }

    fn get_regular_attributes(&self) -> Vec<RegularAttribute> {
        self.attributes
            .iter()
            .filter(|attr| matches!(attr, Attribute::RegularAttribute(_)))
            .map(|attr| match attr {
                Attribute::RegularAttribute(attr) => attr.clone(),
                _ => unreachable!(),
            })
            .collect()
    }

    fn get_for_attribute(&self) -> ForExpr {
        let attr = self
            .attributes
            .iter()
            .find(|attr| matches!(attr, Attribute::For(_)))
            .unwrap();
        match attr {
            Attribute::For(attr) => attr.clone(),
            _ => unreachable!(),
        }
    }
}

impl Component {
    fn has_for_attribute(&self) -> bool {
        self.props
            .iter()
            .any(|attr| matches!(attr, Attribute::For(_)))
    }

    fn get_regular_attributes(&self) -> Vec<RegularAttribute> {
        self.props
            .iter()
            .filter(|attr| matches!(attr, Attribute::RegularAttribute(_)))
            .map(|attr| match attr {
                Attribute::RegularAttribute(attr) => attr.clone(),
                _ => unreachable!(),
            })
            .collect()
    }

    fn get_for_attribute(&self) -> ForExpr {
        let attr = self
            .props
            .iter()
            .find(|attr| matches!(attr, Attribute::For(_)))
            .unwrap();
        match attr {
            Attribute::For(attr) => attr.clone(),
            _ => unreachable!(),
        }
    }
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

        if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            let tag_name: Path = input.parse()?;

            let mut attributes = Vec::new();

            // Parse attributes until closing angle bracket is found
            while !input.peek(Token![>]) && !input.peek2(Token![>]) {
                let attribute: Attribute = input.parse()?;
                attributes.push(attribute);
            }

            let is_component = is_path_pascal_case(&tag_name);

            // Self-closing tag
            if input.peek(Token![/]) && input.peek2(Token![>]) {
                let _: Token![/] = input.parse()?;
                let _: Token![>] = input.parse()?;

                // Self-closing -> no children (slots)
                if is_component {
                    return Ok(Node::Component(Component {
                        name: tag_name,
                        props: attributes,
                        children: Vec::new(),
                    }));
                }

                return Ok(Node::Element(Tag {
                    tag_name: extract_ident_from_path(&tag_name),
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
                tag_name: extract_ident_from_path(&tag_name),
                attributes: attributes.clone(),
                children: children.clone(),
                self_closing: false,
            };

            // Check for the closing tag
            if input.peek(Token![<]) && input.peek2(Token![/]) && input.peek3(Ident) {
                let _: Token![<] = input.parse()?;
                let _: Token![/] = input.parse()?;
                let closing_tag_name: Path = input.parse()?;
                if closing_tag_name != tag_name {
                    Err(input.error(format!(
                        "Expected closing tag {}, found {}",
                        path_to_string(&tag_name),
                        path_to_string(&closing_tag_name)
                    )))
                } else {
                    let _: Token![>] = input.parse()?;

                    if is_component {
                        return Ok(Node::Component(Component {
                            name: tag_name,
                            props: attributes,
                            children,
                        }));
                    }

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

            // Peek ahead to see if there's another pair of braces
            if content_brackets.peek(Brace) {
                // If there's another pair of braces, parse the inner content
                let inner_brackets;
                braced!(inner_brackets in content_brackets);
                let content_expr: Expr = inner_brackets.parse()?;
                Ok(Node::UnescapedExpression(content_expr))
            } else {
                // If there's only one pair of braces, parse the content normally
                let content_expr: Expr = content_brackets.parse()?;
                Ok(Node::Expression(content_expr))
            }
        } else {
            Err(input.error("Expected a node"))
        }
    }
}

fn is_path_pascal_case(path: &Path) -> bool {
    is_pascal_case(&extract_ident_from_path(path))
}

fn extract_ident_from_path(path: &Path) -> Ident {
    path.segments.last().unwrap().ident.clone()
}

fn path_to_string(path: &Path) -> String {
    path.to_token_stream()
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![:]) && input.peek2(Token![for]) {
            let _: Token![:] = input.parse()?;
            let _: Token![for] = input.parse()?;
            let _: Token![=] = input.parse()?;

            let content;
            braced!(content in input);
            return Ok(Attribute::For(content.parse()?));
        }
        let name: AttrName = input.parse()?;

        // If the next token is '=', then expect a value. Otherwise, no value.
        let value = if input.peek(Token![=]) {
            let _: Token![=] = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Attribute::RegularAttribute(RegularAttribute {
            name,
            value,
        }))
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
        let mut saw_word = false;
        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                if saw_word {
                    break;
                }
                let ident: Ident = input.parse()?;
                name.push_str(&ident.to_string());
                saw_word = true;
            } else if lookahead.peek(Token![type]) {
                if saw_word {
                    break;
                }
                // TODO all the rest of keywords
                let _: Token![type] = input.parse()?;
                name.push_str("type");
                saw_word = true;
            } else if lookahead.peek(Token![-]) {
                let _: Token![-] = input.parse()?;
                name.push('-');
                saw_word = false;
            } else if lookahead.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                name.push(':');
                saw_word = false;
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
            if lit_str.value().contains('{') && lit_str.value().contains('}') {
                // Contains interpolation
                let segments = parse_interpolated_string(&lit_str.value())?;
                Ok(AttrValue::Interpolated(segments))
            } else {
                Ok(AttrValue::Literal(lit_str))
            }
        }
    }
}

impl Parse for ForExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let pat: Pat = Pat::parse_single(input)?;
        let _: Token![in] = input.parse()?;
        let collection: Expr = input.parse()?;
        Ok(ForExpr { pat, collection })
    }
}

fn parse_interpolated_string(s: &str) -> Result<Vec<InterpolatedSegment>> {
    let mut segments = Vec::new();
    let mut start = 0;
    while let Some(open) = s[start..].find('{') {
        if start != open {
            segments.push(InterpolatedSegment::Str(LitStr::new(
                &s[start..start + open],
                Span::call_site(),
            )));
        }
        let close = s[start + open..].find('}').ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                "Unmatched opening brace in interpolated string",
            )
        })?;
        let expr_str = &s[start + open + 1..start + open + close];
        let expr: Expr = syn::parse_str(expr_str)?;
        segments.push(InterpolatedSegment::Expr(expr));
        start = start + open + close + 1;
    }
    if start != s.len() {
        segments.push(InterpolatedSegment::Str(LitStr::new(
            &s[start..],
            Span::call_site(),
        )));
    }
    Ok(segments)
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
            if element.has_for_attribute() {
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
            if component.has_for_attribute() {
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
