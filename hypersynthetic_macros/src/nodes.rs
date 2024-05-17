use syn::{Expr, Ident, LitStr, Path};

use crate::attributes::{Attribute, ForExpr, RegularAttribute};

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
    Text(LitStr),
    UnescapedExpression(Expr),
}

#[derive(Clone)]
pub struct Tag {
    pub tag_name: Ident,
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
}

impl Component {
    pub fn has_for_attribute(&self) -> bool {
        self.props
            .iter()
            .any(|attr| matches!(attr, Attribute::For(_)))
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
}
