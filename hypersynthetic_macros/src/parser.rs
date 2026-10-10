use proc_macro2::{Group, Span, TokenStream as TokenStream2, TokenTree};
use syn::{
    Expr, Ident, LitInt, LitStr, Pat, Path, Result, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::{
    attributes::{
        AttrName, AttrValue, Attribute, ForExpr, InterpolatedSegment, InterpolatedString,
        RegularAttribute,
    },
    nodes::{Component, Node, NodeCollection, Tag, TagName},
    utils::{is_path_pascal_case, path_to_string},
};

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) && input.peek2(Token![!]) {
            let _: Token![<] = input.parse()?;
            let _: Token![!] = input.parse()?;
            expect_word(input, "DOCTYPE")?;
            expect_word(input, "html")?;
            input
                .parse::<Token![>]>()
                .map_err(|err| syn::Error::new(err.span(), DOCTYPE_ERROR))?;
            return Ok(Node::DocType);
        }

        if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            let tag_name_span = input.span();
            let tag_name: TagName = input.parse()?;

            let mut attributes = Vec::new();

            let mut end_of_regular_tag = input.peek(Token![>]);
            let mut end_of_self_closing_tag = input.peek(Token![/]) && input.peek2(Token![>]);
            let mut end_of_tag = end_of_regular_tag || end_of_self_closing_tag;

            // Parse attributes until end of tag
            let mut seen_if = false;
            let mut seen_for = false;
            while !end_of_tag {
                let attribute_span = input.span();
                let attribute: Attribute = input.parse()?;
                match attribute {
                    Attribute::If(_) if seen_if => {
                        return Err(syn::Error::new(
                            attribute_span,
                            "duplicate `:if`; combine the conditions with `&&`",
                        ));
                    }
                    Attribute::For(_) if seen_for => {
                        return Err(syn::Error::new(
                            attribute_span,
                            "duplicate `:for`; wrap the element in another one to nest loops",
                        ));
                    }
                    Attribute::If(_) => seen_if = true,
                    Attribute::For(_) => seen_for = true,
                    Attribute::RegularAttribute(_) => {}
                }
                attributes.push(attribute);

                end_of_regular_tag = input.peek(Token![>]);
                end_of_self_closing_tag = input.peek(Token![/]) && input.peek2(Token![>]);
                end_of_tag = end_of_regular_tag || end_of_self_closing_tag;
            }

            if matches!(tag_name, TagName::Component(_)) {
                validate_prop_names(&attributes)?;
            }

            // Self-closing tag
            if input.peek(Token![/]) && input.peek2(Token![>]) {
                let _: Token![/] = input.parse()?;
                let _: Token![>] = input.parse()?;

                // Self-closing -> no children (slots)
                return Ok(match tag_name {
                    TagName::Component(name) => Node::Component(Component {
                        name,
                        props: attributes,
                        children: Vec::new(),
                    }),
                    TagName::Element(tag_name) => Node::Element(Tag {
                        tag_name,
                        attributes,
                        children: Vec::new(),
                        self_closing: true,
                    }),
                });
            }

            let _: Token![>] = input.parse()?;

            let mut children: Vec<Node> = Vec::new();
            while input.peek(Token![<]) && input.peek2(Ident::peek_any)
                || input.peek(LitStr)
                || input.peek(Brace)
            {
                let child: Node = input.parse()?;
                children.push(child);
            }

            // Check for the closing tag
            if input.peek(Token![<]) && input.peek2(Token![/]) && input.peek3(Ident::peek_any) {
                let _: Token![<] = input.parse()?;
                let _: Token![/] = input.parse()?;
                let closing_span = input.span();
                let closing_tag_name: TagName = input.parse()?;
                if closing_tag_name != tag_name {
                    Err(syn::Error::new(
                        closing_span,
                        format!("expected closing tag `{tag_name}`, found `{closing_tag_name}`"),
                    ))
                } else {
                    let _: Token![>] = input.parse()?;

                    if matches!(tag_name, TagName::Component(_)) && !children.is_empty() {
                        reject_children_attribute(&attributes)?;
                    }

                    Ok(match tag_name {
                        TagName::Component(name) => Node::Component(Component {
                            name,
                            props: attributes,
                            children,
                        }),
                        TagName::Element(tag_name) => Node::Element(Tag {
                            children: if is_raw_text_element(&tag_name) {
                                raw_text_children(&tag_name, children)?
                            } else {
                                children
                            },
                            tag_name,
                            attributes,
                            self_closing: false,
                        }),
                    })
                }
            } else if input.is_empty() {
                Err(syn::Error::new(
                    tag_name_span,
                    format!("unclosed `<{tag_name}>`"),
                ))
            } else {
                Err(input.error(format!(
                    "expected a string, `{{expression}}`, a tag, or `</{tag_name}>`"
                )))
            }
        } else if input.peek(LitStr) {
            Ok(Node::Text(input.parse()?))
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

const DOCTYPE_ERROR: &str = "expected `<!DOCTYPE html>`";

/// Parses `word` (case-insensitively), as part of `<!DOCTYPE html>`.
fn expect_word(input: ParseStream, word: &str) -> Result<()> {
    let ident = input
        .call(Ident::parse_any)
        .map_err(|err| syn::Error::new(err.span(), DOCTYPE_ERROR))?;
    if ident.to_string().eq_ignore_ascii_case(word) {
        Ok(())
    } else {
        Err(syn::Error::new(ident.span(), DOCTYPE_ERROR))
    }
}

impl Parse for TagName {
    fn parse(input: ParseStream) -> Result<Self> {
        // Element names can be Rust keywords (SVG's `<use>`), which a path can't
        // be, unless it's a path keyword followed by `::` (`crate::Card`)
        let is_path = input.peek(Ident) || input.peek(Token![::]) || input.peek2(Token![::]);
        let mut name = if is_path {
            let path: Path = input.parse()?;
            if is_path_pascal_case(&path) {
                return Ok(TagName::Component(path));
            }
            // Only components can be paths: `<foo::bar>` isn't an element
            match path.get_ident() {
                Some(ident) => ident.to_string(),
                None => {
                    return Err(syn::Error::new_spanned(
                        &path,
                        format!(
                            "`{}` isn't a valid tag name: element names can't be paths, \
                             and component names start with an uppercase letter",
                            path_to_string(&path)
                        ),
                    ));
                }
            }
        } else {
            input.call(Ident::parse_any)?.to_string()
        };

        // Custom elements have hyphens in their names: `<my-widget>`
        while input.peek(Token![-]) {
            let _: Token![-] = input.parse()?;
            let part = input.call(Ident::parse_any)?;
            name.push('-');
            name.push_str(&part.to_string());
        }
        Ok(TagName::Element(name))
    }
}

/// Elements whose content is JavaScript or CSS, not HTML: browsers don't decode
/// entities there, so HTML escaping would change the code.
/// See <https://html.spec.whatwg.org/multipage/syntax.html#raw-text-elements>.
fn is_raw_text_element(tag_name: &str) -> bool {
    matches!(tag_name, "script" | "style")
}

/// String literals in a raw text element are code, written by the template's author,
/// so they become raw text. Values would need JavaScript or CSS escaping, which
/// `{expression}` can't do, so they must be inserted unescaped with `{{expression}}`.
fn raw_text_children(tag_name: &str, children: Vec<Node>) -> Result<Vec<Node>> {
    let advice = format!(
        "insert the value unescaped with `{{{{...}}}}`, and make sure it can't contain `</{tag_name}>`"
    );
    children
        .into_iter()
        .map(|child| match child {
            Node::Text(text) => {
                let mut raw = String::new();
                for segment in text.segments {
                    match segment {
                        InterpolatedSegment::Str(part) => raw.push_str(&part),
                        InterpolatedSegment::Expr { .. } => {
                            return Err(syn::Error::new(
                                text.lit.span(),
                                format!(
                                    "`<{tag_name}>` content isn't HTML, so `{{...}}` in a string \
                                     can't escape values for it: close the string and {advice}"
                                ),
                            ));
                        }
                    }
                }
                Ok(Node::RawText(raw))
            }
            Node::Expression(expr) => Err(syn::Error::new_spanned(
                expr,
                format!(
                    "`<{tag_name}>` content isn't HTML, so `{{...}}` can't escape values for it: {advice}"
                ),
            )),
            other => Ok(other),
        })
        .collect()
}

/// Props become builder method calls (`.data_id(…)`), so their names must be
/// Rust identifiers, unlike HTML attribute names which can contain `-`, `:`, `@`
/// and `.`.
fn validate_prop_names(props: &[Attribute]) -> Result<()> {
    for prop in props {
        let Attribute::RegularAttribute(RegularAttribute { name, .. }) = prop else {
            continue;
        };
        match name {
            AttrName::Expression(expr) => {
                return Err(syn::Error::new_spanned(
                    expr,
                    "component prop names can't be expressions, write the prop name literally",
                ));
            }
            AttrName::Literal(name) => {
                let name_str = name.value();
                let is_word = |c: char| c.is_alphanumeric() || c == '_';
                if !name_str.chars().all(is_word) {
                    let words: Vec<&str> = name_str
                        .split(|c| !is_word(c))
                        .filter(|word| !word.is_empty())
                        .collect();
                    let suggestion = words.join("_");
                    return Err(syn::Error::new(
                        name.span(),
                        format!(
                            "invalid prop name `{name_str}`: component props are Rust identifiers, try `{suggestion}`"
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

/// The content between a component's tags is passed as its `children` prop,
/// so it can't also be given as an attribute.
fn reject_children_attribute(props: &[Attribute]) -> Result<()> {
    let children_attribute = props.iter().find_map(|prop| match prop {
        Attribute::RegularAttribute(RegularAttribute {
            name: AttrName::Literal(name),
            ..
        }) if name.value() == "children" => Some(name),
        _ => None,
    });
    match children_attribute {
        Some(name) => Err(syn::Error::new(
            name.span(),
            "`children` is given twice: as an attribute and as the content between the tags",
        )),
        None => Ok(()),
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![:]) {
            if input.peek2(Token![for]) {
                let _: Token![:] = input.parse()?;
                let _: Token![for] = input.parse()?;
                let _: Token![=] = input.parse()?;

                let content;
                braced!(content in input);
                return Ok(Attribute::For(content.parse()?));
            } else if input.peek2(Token![if]) {
                let _: Token![:] = input.parse()?;
                let _: Token![if] = input.parse()?;
                let _: Token![=] = input.parse()?;

                let content;
                braced!(content in input);
                return Ok(Attribute::If(content.parse()?));
            }
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
            // Any identifier, including Rust keywords (`type`, `for`, `async`) and
            // `true`/`false`, which are all identifiers at the token level
            if lookahead.peek(Ident::peek_any) {
                if saw_word {
                    break;
                }
                let ident = input.call(Ident::parse_any)?;
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
            // Alpine.js: `@click`, `x-on:submit.prevent`
            } else if lookahead.peek(Token![@]) {
                let _: Token![@] = input.parse()?;
                name.push('@');
                saw_word = false;
            } else if lookahead.peek(Token![.]) {
                let _: Token![.] = input.parse()?;
                name.push('.');
                saw_word = false;
            // Numbers, with suffixes: `data-2`, `.debounce.500ms`
            } else if lookahead.peek(LitInt) {
                if saw_word {
                    break;
                }
                let token: LitInt = input.parse()?;
                name.push_str(&token.to_string());
                saw_word = true;
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
            let string: InterpolatedString = input.parse()?;
            let span = string.lit.span();
            match string.segments.as_slice() {
                [] => Ok(AttrValue::Literal(LitStr::new("", span))),
                [InterpolatedSegment::Str(text)] => Ok(AttrValue::Literal(LitStr::new(text, span))),
                _ => Ok(AttrValue::Interpolated(string)),
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

/// Splits a string literal into text and `{expression}`/`{expression:spec}` parts.
/// `{{` and `}}` are literal braces, as in `format!` strings.
impl Parse for InterpolatedString {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: LitStr = input.parse()?;
        let value = lit.value();
        let error = |message: String| syn::Error::new(lit.span(), message);

        let mut segments = Vec::new();
        let mut is_format_string = true;
        let mut text = String::new();
        let mut chars = value.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '{' if chars.next_if_eq(&'{').is_some() => text.push('{'),
                '}' if chars.next_if_eq(&'}').is_some() => text.push('}'),
                '{' => {
                    let mut content = String::new();
                    loop {
                        match chars.next() {
                            Some('}') => break,
                            Some(c) => content.push(c),
                            None => {
                                return Err(error(
                                    "unmatched `{`; use `{{` for a literal brace".to_owned(),
                                ));
                            }
                        }
                    }
                    let (expr, spec) =
                        parse_interpolation(&content, lit.span()).ok_or_else(|| {
                            error(format!(
                                "invalid expression `{{{content}}}`; \
                             use `{{{{` and `}}}}` for literal braces"
                            ))
                        })?;
                    let expr_text = match &spec {
                        Some(spec) => &content[..content.len() - spec.len() - 1],
                        None => &content,
                    };
                    is_format_string &= is_identifier(&expr, expr_text);
                    if !text.is_empty() {
                        segments.push(InterpolatedSegment::Str(std::mem::take(&mut text)));
                    }
                    segments.push(InterpolatedSegment::Expr { expr, spec });
                }
                '}' => {
                    return Err(error(
                        "unmatched `}`; use `}}` for a literal brace".to_owned(),
                    ));
                }
                c => text.push(c),
            }
        }
        if !text.is_empty() {
            segments.push(InterpolatedSegment::Str(text));
        }
        Ok(InterpolatedString {
            lit,
            segments,
            is_format_string,
        })
    }
}

/// Parses `expression` or `expression:spec`. The spec starts after the last
/// `:` that isn't part of a `::` path separator.
fn parse_interpolation(content: &str, span: Span) -> Option<(Expr, Option<String>)> {
    if let Ok(expr) = parse_expr_with_span(content, span) {
        return Some((expr, None));
    }
    let bytes = content.as_bytes();
    let colon = (0..bytes.len()).rev().find(|&i| {
        bytes[i] == b':' && bytes.get(i + 1) != Some(&b':') && (i == 0 || bytes[i - 1] != b':')
    })?;
    let spec = &content[colon + 1..];
    if !is_format_spec(spec) {
        return None;
    }
    let expr = parse_expr_with_span(&content[..colon], span).ok()?;
    Some((expr, Some(spec.to_owned())))
}

/// Checks the `std::fmt` grammar, so that text after a `:` that isn't a spec
/// (`{ open: false }`) isn't mistaken for one:
/// `[[fill]align][sign]['#']['0'][width]['.' precision][type]`
fn is_format_spec(spec: &str) -> bool {
    fn is_align(c: char) -> bool {
        matches!(c, '<' | '^' | '>')
    }
    fn is_word(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
    // count := integer | (integer | identifier) '$'
    fn strip_count(s: &str) -> &str {
        let word = s.find(|c| !is_word(c)).unwrap_or(s.len());
        if word > 0 && s[word..].starts_with('$') {
            return &s[word + 1..];
        }
        s.trim_start_matches(|c: char| c.is_ascii_digit())
    }

    let mut s = spec;
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(fill), Some(align)) if is_align(align) => s = &s[fill.len_utf8() + 1..],
        (Some(align), _) if is_align(align) => s = &s[1..],
        _ => {}
    }
    s = s.strip_prefix(['+', '-']).unwrap_or(s);
    s = s.strip_prefix('#').unwrap_or(s);
    s = s.strip_prefix('0').unwrap_or(s);
    s = strip_count(s);
    if let Some(precision) = s.strip_prefix('.') {
        s = precision
            .strip_prefix('*')
            .unwrap_or_else(|| strip_count(precision));
    }
    // type: empty, `?`, `x?`, `X?`, or a trait name like `x` or `e`
    let ty = s.strip_suffix('?').unwrap_or(s);
    ty.is_empty() || (ty.chars().all(is_word) && !ty.starts_with(|c: char| c.is_ascii_digit()))
}

/// Tokens parsed from a string get `Span::call_site()`, so errors in them would
/// point at the whole macro. Give them the span of the literal they came from.
fn parse_expr_with_span(code: &str, span: Span) -> Result<Expr> {
    fn respan(tokens: TokenStream2, span: Span) -> TokenStream2 {
        tokens
            .into_iter()
            .map(|token| match token {
                TokenTree::Group(group) => {
                    let mut new = Group::new(group.delimiter(), respan(group.stream(), span));
                    new.set_span(span);
                    TokenTree::Group(new)
                }
                mut other => {
                    other.set_span(span);
                    other
                }
            })
            .collect()
    }
    let tokens: TokenStream2 = syn::parse_str(code)?;
    syn::parse2(respan(tokens, span))
}

/// Whether `format!` would accept `text` as an inline argument: a plain
/// identifier, written without whitespace.
fn is_identifier(expr: &Expr, text: &str) -> bool {
    matches!(expr, Expr::Path(path) if path.qself.is_none() && path.path.get_ident().is_some())
        && text.chars().all(|c| c.is_alphanumeric() || c == '_')
}
