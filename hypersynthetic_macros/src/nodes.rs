use std::fmt;

use syn::{Expr, Path};

use crate::{
    attributes::{Attribute, ForExpr, InterpolatedString, LiteralName, RegularAttribute},
    utils::path_to_string,
};

#[derive(Clone)]
pub enum NodeCollection {
    Nodes(Vec<Node>),
}

#[derive(Clone)]
pub enum Node {
    Component(Component),
    DocType,
    Element(Tag),
    Expression(Expr),
    /// Text of a `<script>` or `<style>`, rendered without escaping
    RawText(String),
    Text(InterpolatedString),
    UnescapedExpression(Expr),
}

/// The name in `<name ...>`: a component path (`Card`, `ui::Card`) or an HTML
/// element name, which can contain hyphens (`div`, `my-widget`).
pub enum TagName {
    Component(Path),
    Element(LiteralName),
}

impl TagName {
    /// Whether `</closing>` ends this tag. Element names are compared ignoring ASCII
    /// case, as an HTML parser does: `</dIV>` closes `<div>`. Component names are
    /// Rust paths, so their names are compared exactly, but a closing tag can leave
    /// the type arguments out: `</Frame>` closes `<Frame::<u8>>`.
    ///
    /// There's no `==` for tag names on purpose: it would be the exact comparison,
    /// which is wrong for both.
    pub fn is_closed_by(&self, closing: &TagName) -> bool {
        match (self, closing) {
            (TagName::Element(name), TagName::Element(closing)) => {
                name.value().eq_ignore_ascii_case(&closing.value())
            }
            (TagName::Component(path), TagName::Component(closing)) => {
                is_same_component(path, closing)
            }
            _ => false,
        }
    }

    /// An error about this name, underlining all of it: `my-widget` and `ui::Card`
    /// are several tokens. Every error about a tag's name is made here, so that
    /// none of them has to know where a name ends.
    pub fn error(&self, message: impl fmt::Display) -> syn::Error {
        match self {
            // A path has its tokens, and `new_spanned` covers them from the first
            // to the last
            TagName::Component(path) => syn::Error::new_spanned(path, message),
            TagName::Element(name) => name.error(message),
        }
    }
}

/// Whether `closing`, the path in a closing tag, names the component of `opening`:
/// the same path, with the same type arguments or without them.
///
/// A closing tag is there to say which tag ends, and the path says that. The type
/// arguments say how the component is called, like the props, which a closing tag
/// doesn't repeat either. Written anyway, they have to be the ones of the opening
/// tag: `</Frame::<u8>>` after `<Frame::<char>>` is a mistake one way or the other.
fn is_same_component(opening: &Path, closing: &Path) -> bool {
    // `zip` stops at the end of the shorter path, so the lengths are compared first:
    // without that, `</Ui>` would close `<Ui::Card>`
    opening.leading_colon.is_some() == closing.leading_colon.is_some()
        && opening.segments.len() == closing.segments.len()
        && opening
            .segments
            .iter()
            .zip(&closing.segments)
            .all(|(opening, closing)| {
                opening.ident == closing.ident
                    && (closing.arguments.is_none() || closing.arguments == opening.arguments)
            })
}

impl fmt::Display for TagName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagName::Component(path) => f.write_str(&path_to_string(path)),
            TagName::Element(name) => f.write_str(&name.value()),
        }
    }
}

#[derive(Clone)]
pub struct Tag {
    pub name: LiteralName,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
    pub self_closing: bool,
}

#[derive(Clone)]
pub struct Component {
    pub name: Path,
    pub props: Vec<Attribute>,
    pub children: Vec<Node>,
}

/// The `:for` pseudo-attribute, if there is one. The parser rejects duplicates.
pub fn for_attribute(attributes: &[Attribute]) -> Option<&ForExpr> {
    attributes.iter().find_map(|attr| match attr {
        Attribute::For(for_expr) => Some(for_expr.as_ref()),
        _ => None,
    })
}

/// The `:if` pseudo-attribute's condition, if there is one. The parser rejects duplicates.
pub fn if_attribute(attributes: &[Attribute]) -> Option<&Expr> {
    attributes.iter().find_map(|attr| match attr {
        Attribute::If(condition) => Some(condition),
        _ => None,
    })
}

/// The attributes that aren't `:for` or `:if`.
pub fn regular_attributes(attributes: &[Attribute]) -> impl Iterator<Item = &RegularAttribute> {
    attributes.iter().filter_map(|attr| match attr {
        Attribute::RegularAttribute(attr) => Some(attr),
        _ => None,
    })
}
