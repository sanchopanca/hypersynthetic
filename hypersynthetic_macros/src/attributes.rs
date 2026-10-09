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
    Interpolated(InterpolatedString),
}

/// A string literal containing `{expression}` or `{expression:spec}`.
#[derive(Clone)]
pub struct InterpolatedString {
    pub lit: LitStr,
    pub segments: Vec<InterpolatedSegment>,
    /// Every interpolation is a plain identifier, so `lit` is itself a valid
    /// `format!` string.
    pub is_format_string: bool,
}

#[derive(Clone)]
pub enum InterpolatedSegment {
    Str(String),
    Expr { expr: Expr, spec: Option<String> },
}

#[derive(Clone)]
pub struct ForExpr {
    pub pat: Pat,
    pub collection: Expr,
}
