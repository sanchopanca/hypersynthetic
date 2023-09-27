extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::{Brace, Comma},
    Block, Expr, FnArg, Ident, ItemFn, LitStr, Pat, PatType, Result, Token, Type,
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
    name: Ident,
    props: Vec<Attribute>,
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
    Identifier(Ident),
}

impl AttrName {
    pub fn to_indent(&self) -> Ident {
        match self {
            AttrName::Literal(s) => panic!("Cannot convert {} to Ident", s.value()),
            AttrName::Expression(_) => panic!("Connot convert expression to Ident, dynamic attribute names are not supported here"),
            AttrName::Identifier(i) => i.to_owned(),
        }
    }
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
        let mut only_saw_ident = true;
        let mut last_ident = None;
        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                if saw_word {
                    break;
                }
                let ident: Ident = input.parse()?;
                name.push_str(&ident.to_string());
                saw_word = true;
                last_ident = Some(ident);
            } else if lookahead.peek(Token![type]) {
                if saw_word {
                    break;
                }
                // TODO all the rest of keywords
                let _: Token![type] = input.parse()?;
                name.push_str("type");
                saw_word = true;
                only_saw_ident = false;
            } else if lookahead.peek(Token![-]) {
                let _: Token![-] = input.parse()?;
                name.push('-');
                saw_word = false;
                only_saw_ident = false;
            } else if lookahead.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                name.push(':');
                saw_word = false;
                only_saw_ident = false;
            } else {
                break;
            }
        }

        if name.is_empty() {
            Err(input.error("Expected a valid attribute name"))
        } else if only_saw_ident {
            Ok(AttrName::Identifier(last_ident.unwrap()))
        } else {
            Ok(AttrName::Literal(LitStr::new(&name, span)))
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

    let companion = create_component_companion(&function);

    quote! {
        #function
        #companion
    }
    .into()
}

// This is one lazy implementation that only checks that the first character is uppercase
fn is_pascal_case(name: &Ident) -> bool {
    let first_char = name.to_string().chars().next();
    matches!(first_char, Some(ch) if ch.is_uppercase())
}

fn create_component_companion(function: &ItemFn) -> TokenStream2 {
    // 1. create new struct based on the function signature
    let struct_name = format_ident!("{}Args", function.sig.ident);

    // Check if any argument is a reference
    let has_lifetime = function.sig.inputs.iter().any(|arg| {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            matches!(**ty, Type::Reference(_))
        } else {
            false
        }
    });

    // Generate the struct fields without types
    let struct_fields: Vec<_> = function
        .sig
        .inputs
        .iter()
        .map(|arg| {
            if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                let ty = match **ty {
                    Type::Reference(ref reference_type) => {
                        let inner_type = &reference_type.elem;
                        quote! { &'a #inner_type }
                    }
                    _ => quote! { #ty },
                };
                quote! {
                    pub #pat: #ty,
                }
            } else {
                panic!("Expected typed arguments!")
            }
        })
        .collect();

    // If there's a lifetime, include it in the struct definition
    let lifetime_def = if has_lifetime {
        quote! {<'a>}
    } else {
        quote! {}
    };
    let strukt = quote! {
        pub struct #struct_name #lifetime_def {
            #(#struct_fields)*
        }
    };

    let args = function.sig.inputs.clone();
    let mut function = function.clone();
    // 2. change the signature to use the struct, unpacking the struct in the signature
    let destructured_patterns: Vec<_> = args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                let pat = &pat_type.pat;
                quote! { #pat }
            } else {
                panic!("Expected typed arguments!")
            }
        })
        .collect();

    // Construct the function signature with the destructured arguments.
    let inputs = quote! { #struct_name { #(#destructured_patterns),* }: #struct_name };
    function.sig.inputs = parse_quote!(#inputs);

    // 3. change the body to call the original function

    // Extract just the variable names from the args.
    let mut vars: Punctuated<Ident, Comma> = Punctuated::new();
    for arg in args.iter() {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                vars.push(pat_ident.ident.clone());
            }
        }
    }

    // Construct a function call using the extracted variable names.
    let fn_name = &function.sig.ident;
    let fn_call = quote! {
        #fn_name(#vars)
    };

    // Create a new block with just the function call as its body.
    let new_body: Block = parse_quote! {
        {
            #fn_call
        }
    };

    // Replace the original function's body with the new body.
    function.block = Box::new(new_body);

    // let output = quote! { #function };
    // println!("We changed the body:\n{}", output);

    // 4. change the name of the function
    function.sig.ident = Ident::new(
        &format!("{}Companion", function.sig.ident),
        function.sig.ident.span(),
    );

    // let output = quote! { #function };
    // println!("We changed the name:\n{}", output);

    // 5. output the struct and the function
    quote! {
        #strukt
        #function
    }
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
        Node::DocType => {
            quote! {
                vec![hypersynthetic::Node::DocType]
            }
        }
        Node::Component(component) => {
            let component_name = format_ident!("{}Companion", &component.name);

            let args_struct_name = format_ident!("{}Args", &component.name);

            let props_initializations: Vec<TokenStream2> = component
                .get_regular_attributes()
                .into_iter()
                .map(|attr| {
                    let attr_name = attr.name.to_indent();
                    let prop_value = generate_attribute_as_prop(attr);
                    quote! {
                        #attr_name: #prop_value
                    }
                })
                .collect();

            let struct_initialization = quote! {
                #args_struct_name {
                    #(#props_initializations),*
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
                            for_v.extend(#component_name(#struct_initialization).get_nodes());
                        }
                        for_v
                    }
                }
            } else {
                quote! {
                    #component_name(#struct_initialization).get_nodes()
                }
            }

            // let props: Vec<TokenStream2> = component
            //     .get_regular_attributes()
            //     .into_iter()
            //     .map(generate_attribute_as_prop)
            //     .collect();
            // if component.has_for_attribute() {
            //     let for_expr = component.get_for_attribute();
            //     let var = for_expr.pat;
            //     let collection = for_expr.collection;
            //     quote! {
            //         {
            //             let mut for_v = Vec::new();
            //             for #var in #collection {
            //                 for_v.extend(#component_name(#(#props),*).get_nodes());
            //             }
            //             for_v
            //         }
            //     }
            // } else {
            //     quote! {
            //         #component_name(#(#props),*).get_nodes()
            //     }
            // }
        }
    }
}

fn generate_attribute(attr: RegularAttribute) -> TokenStream2 {
    let attr_name = match &attr.name {
        AttrName::Literal(name) => quote! { #name.to_owned() },
        AttrName::Expression(expr) => {
            quote! { hypersynthetic::escape_attribute(format!("{}", #expr)).to_string() }
        }
        AttrName::Identifier(ident) => {
            let name = ident.to_string();
            quote! { #name.to_owned() }
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
