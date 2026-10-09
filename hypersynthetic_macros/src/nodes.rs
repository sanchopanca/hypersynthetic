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

impl Tag {
    pub fn has_for_attribute(&self) -> bool {
        self.attributes
            .iter()
            .any(|attr| matches!(attr, Attribute::For(_)))
    }

    pub fn has_if_attribute(&self) -> bool {
        self.attributes
            .iter()
            .any(|attr| matches!(attr, Attribute::If(_)))
    }

    pub fn get_regular_attributes(&self) -> Vec<RegularAttribute> {
        self.attributes
            .iter()
            .filter(|attr| matches!(attr, Attribute::RegularAttribute(_)))
            .map(|attr| match attr {
                Attribute::RegularAttribute(attr) => attr.clone(),
                _ => unreachable!(),
            })
            .collect()
    }

    pub fn get_for_attribute(&self) -> ForExpr {
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

    pub fn get_if_attribute(&self) -> Expr {
        let attr = self
            .attributes
            .iter()
            .find(|attr| matches!(attr, Attribute::If(_)))
            .unwrap();
        match attr {
            Attribute::If(attr) => attr.clone(),
            _ => unreachable!(),
        }
    }
}

impl Component {
    pub fn has_for_attribute(&self) -> bool {
        self.props
            .iter()
            .any(|attr| matches!(attr, Attribute::For(_)))
    }

    pub fn has_if_attribute(&self) -> bool {
        self.props
            .iter()
            .any(|attr| matches!(attr, Attribute::If(_)))
    }

    pub fn get_regular_attributes(&self) -> Vec<RegularAttribute> {
        self.props
            .iter()
            .filter(|attr| matches!(attr, Attribute::RegularAttribute(_)))
            .map(|attr| match attr {
                Attribute::RegularAttribute(attr) => attr.clone(),
                _ => unreachable!(),
            })
            .collect()
    }

    pub fn get_for_attribute(&self) -> ForExpr {
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

    pub fn get_if_attribute(&self) -> Expr {
        let attr = self
            .props
            .iter()
            .find(|attr| matches!(attr, Attribute::If(_)))
            .unwrap();
        match attr {
            Attribute::If(attr) => attr.clone(),
            _ => unreachable!(),
        }
    }
}
