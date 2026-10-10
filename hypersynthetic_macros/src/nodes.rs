use std::fmt;

use syn::{Expr, Path};

use crate::{
    attributes::{Attribute, ForExpr, InterpolatedString, RegularAttribute},
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
#[derive(PartialEq)]
pub enum TagName {
    Component(Path),
    Element(String),
}

impl fmt::Display for TagName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagName::Component(path) => f.write_str(&path_to_string(path)),
            TagName::Element(name) => f.write_str(name),
        }
    }
}

#[derive(Clone)]
pub struct Tag {
    pub tag_name: String,
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
        Attribute::For(for_expr) => Some(for_expr),
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
