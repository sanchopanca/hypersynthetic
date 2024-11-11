use syn::{Expr, LitStr, Pat};

#[derive(Clone)]
pub enum Attribute {
    #[allow(clippy::enum_variant_names)]
    RegularAttribute(RegularAttribute),
    For(ForExpr),
    If(Expr),
}

#[derive(Clone)]
pub struct RegularAttribute {
    pub name: AttrName,
    pub value: Option<AttrValue>,
}

#[derive(Clone)]
pub enum AttrName {
    Literal(LitStr),
    Expression(Expr),
}

#[derive(Clone)]
pub enum AttrValue {
    Literal(LitStr),
    Expression(Expr),
    Interpolated(Vec<InterpolatedSegment>),
}

#[derive(Clone)]
pub enum InterpolatedSegment {
    Str(LitStr),
    Expr(Expr),
}

#[derive(Clone)]
pub struct ForExpr {
    pub pat: Pat,
    pub collection: Expr,
}
