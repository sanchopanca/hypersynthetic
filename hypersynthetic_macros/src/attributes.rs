use syn::{Expr, LitStr, Pat};

#[derive(Clone)]
pub enum Attribute {
    #[allow(clippy::enum_variant_names)]
    RegularAttribute(RegularAttribute),
    // Boxed: an enum is as big as its biggest variant, and a pattern plus an
    // expression is about twice the size of the other variants
    For(Box<ForExpr>),
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
    // Boxed: an `Expr` is some 30 times bigger than a `LitStr`, and every name
    // would take that much space
    Expression(Box<Expr>),
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

impl InterpolatedString {
    /// The text, if it has no `{expression}` in it. `{{` and `}}` are already
    /// single braces.
    pub fn literal(&self) -> Option<String> {
        self.segments
            .iter()
            .map(|segment| match segment {
                InterpolatedSegment::Str(text) => Some(text.as_str()),
                InterpolatedSegment::Expr { .. } => None,
            })
            .collect()
    }
}

#[derive(Clone)]
pub enum InterpolatedSegment {
    Str(String),
    Expr {
        expr: Box<Expr>,
        spec: Option<String>,
    },
}

#[derive(Clone)]
pub struct ForExpr {
    pub pat: Pat,
    pub collection: Expr,
}
