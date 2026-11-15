use std::fmt::Display;

use proc_macro2::Span;
use quote::quote;
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
    Literal(LiteralName),
    // Boxed: an `Expr` is some 30 times bigger than a `LitStr`, and every name
    // would take that much space
    Expression(Box<Expr>),
}

/// A name written in the template: an attribute's (`class`, `data-id`, `@click`,
/// `x-on:click.prevent`) or an element's (`div`, `my-widget`).
///
/// For Rust it's several tokens (`data`, `-`, `id`), which the parser puts together
/// into one string. Where the name is in the source is kept as its two ends.
#[derive(Clone)]
pub struct LiteralName {
    /// The name, placed at its first token
    pub lit: LitStr,
    /// Where its last token is
    end: Span,
}

impl LiteralName {
    pub fn new(name: &str, start: Span, end: Span) -> Self {
        LiteralName {
            lit: LitStr::new(name, start),
            end,
        }
    }

    pub fn value(&self) -> String {
        self.lit.value()
    }

    /// An error about this name, underlining all of it.
    ///
    /// Two spans can't be joined into one on stable Rust. An error can still cover
    /// both: `new_spanned` places it from the first of the tokens it's given to the
    /// last, so it gets the name twice, once at each end.
    pub fn error(&self, message: impl Display) -> syn::Error {
        let start = &self.lit;
        let mut end = self.lit.clone();
        end.set_span(self.end);
        syn::Error::new_spanned(quote! { #start #end }, message)
    }
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
