use proc_macro2::Span;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
    Expr, Ident, LitBool, LitStr, Pat, Path, Result, Token,
};

use crate::{
    attributes::{AttrName, AttrValue, Attribute, ForExpr, InterpolatedSegment, RegularAttribute},
    nodes::{Component, Node, NodeCollection, Tag},
    utils::{extract_ident_from_path, is_path_pascal_case, path_to_string},
};

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

macro_rules! match_keyword {
    ($input:expr, $keyword:ident, $name:expr, $saw_word:expr) => {
        if $input.peek(Token![$keyword]) {
            if $saw_word {
                break;
            }
            let _: Token![$keyword] = $input.parse()?;
            $name.push_str(stringify!($keyword));
            $saw_word = true;
            continue;
        }
    };
}

macro_rules! match_keywords {
    ($input:expr, $name:expr, $saw_word:expr, [$($keyword:ident),*]) => {
        $(
            match_keyword!($input, $keyword, $name, $saw_word);
        )*
    };
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
            } else if lookahead.peek(Token![-]) {
                let _: Token![-] = input.parse()?;
                name.push('-');
                saw_word = false;
            } else if lookahead.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                name.push(':');
                saw_word = false;
            // true and false literals
            } else if lookahead.peek(LitBool) {
                let token: LitBool = input.parse()?;
                name.push_str(&token.value.to_string());
                saw_word = true;
            // rest of the keywords
            } else {
                match_keywords!(input, name, saw_word, [
                    // Strict Keywords
                    as,
                    break,
                    const,
                    continue,
                    crate,
                    else,
                    enum,
                    extern,
                    fn,
                    for,
                    if,
                    impl,
                    in,
                    let,
                    loop,
                    match,
                    mod,
                    move,
                    mut,
                    pub,
                    ref,
                    return,
                    self,
                    Self,
                    static,
                    struct,
                    super,
                    trait,
                    type,
                    unsafe,
                    use,
                    where,
                    while,
                    // Strict Keywords 2018 Edition
                    async,
                    await,
                    dyn,
                    // Reserved Keywords
                    abstract,
                    become,
                    box,
                    do,
                    final,
                    macro,
                    override,
                    priv,
                    typeof,
                    unsized,
                    virtual,
                    yield,
                    // Reserved Keywords 2018 Edition
                    try,
                    // Weak Keywords
                    union
                ]);

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

impl Parse for NodeCollection {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            nodes.push(input.parse()?);
        }
        Ok(NodeCollection::Nodes(nodes))
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
