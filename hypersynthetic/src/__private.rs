//! Support code for the macros, not public API.
//!
//! `{expr}` in `html!` inserts `expr` as nodes when it's HTML ([AsNodes]: a fragment,
//! a node or an element, however it's wrapped), and formats it with [Display]
//! otherwise. Trait impls can't express that on stable Rust, since the HTML types
//! implement `Display` too, so the macro uses autoref specialization: it generates
//! `(&Render(&expr)).render(escape, &mut nodes)`. Method lookup tries the receiver
//! `&Render<T>` first, which only the [RenderFragment] impl accepts, and then
//! `&&Render<T>`, which the [RenderDisplay] and [RenderOption] impls accept.
//!
//! An `Option` renders its value like the value itself would, and nothing for `None`.
//! An option of HTML is HTML itself (it's [AsNodes]), so it's taken in the first
//! step: the generic `Option<T: Display>` impl of the second would format it.
//!
//! `Render` holds a reference, so expressions are borrowed like `format!` borrows them.
//!
//! An attribute value `attr={expr}` uses the same lookup with `attribute_value(name)`,
//! which gives `None` to leave the attribute out, or `Some` of its value. [RenderBool]
//! takes a `bool` first, however it's wrapped ([AsBool]): on a boolean attribute
//! (`disabled`) it decides whether the attribute is there. [RenderFragment] has no
//! such method, so any other `Display` value (a fragment too) is formatted, and an
//! `Option` is left out when `None`.

use std::borrow::Cow;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;

use crate::{ElementData, HtmlFragment, Node, escape_text, is_boolean_attribute};

/// For the `TypedBuilder` derive that `#[component]` generates, since users don't
/// depend on typed-builder. It's here rather than public so that typed-builder's
/// version isn't part of hypersynthetic's API.
pub use typed_builder;

pub struct Render<'a, T: ?Sized>(pub &'a T);

fn text(value: Option<&dyn Display>, escape: bool, out: &mut Vec<Node>) {
    let Some(value) = value else {
        return;
    };
    let text = value.to_string();
    // Escaping an owned String gives an owned Cow, which can be 'static
    let text = if escape {
        escape_text(text)
    } else {
        Cow::Owned(text)
    };
    out.push(Node::Text(text));
}

/// Values that are HTML. `{value}` inserts their nodes, where any other value is
/// formatted and escaped.
///
/// The three types that hold nodes implement it, and so does every wrapper around
/// something that does, which covers the combinations without listing them:
/// `&&HtmlFragment`, `Option<&ElementData>`, `&Option<Arc<HtmlFragment>>`, ...
pub trait AsNodes {
    fn push_nodes(&self, out: &mut Vec<Node>);
}

impl AsNodes for HtmlFragment {
    fn push_nodes(&self, out: &mut Vec<Node>) {
        out.extend(self.iter().cloned());
    }
}

impl AsNodes for Node {
    fn push_nodes(&self, out: &mut Vec<Node>) {
        out.push(self.clone());
    }
}

impl AsNodes for ElementData {
    fn push_nodes(&self, out: &mut Vec<Node>) {
        out.push(Node::Element(self.clone()));
    }
}

impl<T: AsNodes> AsNodes for Option<T> {
    fn push_nodes(&self, out: &mut Vec<Node>) {
        if let Some(value) = self {
            value.push_nodes(out);
        }
    }
}

impl<T: AsNodes + ToOwned + ?Sized> AsNodes for Cow<'_, T> {
    fn push_nodes(&self, out: &mut Vec<Node>) {
        (**self).push_nodes(out);
    }
}

/// Values that are a `bool`. As an attribute's value, `attr={value}`, they're not
/// formatted like other values: on a boolean attribute they decide whether it's there.
///
/// Like [AsNodes], it's implemented for the type itself and for every wrapper around
/// something that implements it: `&&bool`, `Option<bool>`, `&Option<Box<bool>>`, ...
pub trait AsBool {
    /// `None` when there's no value: an `Option` that is `None`, at any depth
    fn as_bool(&self) -> Option<bool>;
}

impl AsBool for bool {
    fn as_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

impl<T: AsBool> AsBool for Option<T> {
    fn as_bool(&self) -> Option<bool> {
        self.as_ref()?.as_bool()
    }
}

impl<T: AsBool + ToOwned + ?Sized> AsBool for Cow<'_, T> {
    fn as_bool(&self) -> Option<bool> {
        (**self).as_bool()
    }
}

/// The wrappers that only point to a value are what the value is: HTML, or a `bool`.
macro_rules! through_pointers {
    ($($pointer:ty),* $(,)?) => {$(
        impl<T: AsNodes + ?Sized> AsNodes for $pointer {
            fn push_nodes(&self, out: &mut Vec<Node>) {
                (**self).push_nodes(out);
            }
        }

        impl<T: AsBool + ?Sized> AsBool for $pointer {
            fn as_bool(&self) -> Option<bool> {
                (**self).as_bool()
            }
        }
    )*};
}

through_pointers!(&T, &mut T, Box<T>, Rc<T>, Arc<T>);

pub trait RenderFragment {
    fn render(&self, escape: bool, out: &mut Vec<Node>);
}

impl<T: AsNodes + ?Sized> RenderFragment for Render<'_, T> {
    fn render(&self, _escape: bool, out: &mut Vec<Node>) {
        self.0.push_nodes(out);
    }
}

/// The value of an attribute that's there: `None` for one without a value (`disabled`)
pub type AttributeValue = Option<Cow<'static, str>>;

/// `attr={flag}`: on a boolean attribute, present without a value or left out. On
/// others, `"true"` or `"false"`, which `aria-expanded` and the like need.
fn bool_attribute(value: bool, name: &str) -> Option<AttributeValue> {
    if is_boolean_attribute(name) {
        value.then_some(None)
    } else {
        Some(Some(Cow::Borrowed(if value { "true" } else { "false" })))
    }
}

pub trait RenderBool {
    fn attribute_value(&self, name: &str) -> Option<AttributeValue>;
}

impl<T: AsBool + ?Sized> RenderBool for Render<'_, T> {
    fn attribute_value(&self, name: &str) -> Option<AttributeValue> {
        // No value, no attribute: `None` is left out of any attribute
        bool_attribute(self.0.as_bool()?, name)
    }
}

pub trait RenderDisplay {
    fn render(&self, escape: bool, out: &mut Vec<Node>);
    fn attribute_value(&self, name: &str) -> Option<AttributeValue>;
}

impl<T: Display + ?Sized> RenderDisplay for &Render<'_, T> {
    fn render(&self, escape: bool, out: &mut Vec<Node>) {
        text(Some(&self.0), escape, out)
    }

    fn attribute_value(&self, _name: &str) -> Option<AttributeValue> {
        Some(Some(Cow::Owned(self.0.to_string())))
    }
}

// A separate trait from RenderDisplay: in one trait, the impls for `T: Display`
// and `Option<T>` would conflict, because std could implement Display for Option.
pub trait RenderOption {
    fn render(&self, escape: bool, out: &mut Vec<Node>);
    fn attribute_value(&self, name: &str) -> Option<AttributeValue>;
}

impl<T: Display> RenderOption for &Render<'_, Option<T>> {
    fn render(&self, escape: bool, out: &mut Vec<Node>) {
        text(
            self.0.as_ref().map(|value| value as &dyn Display),
            escape,
            out,
        )
    }

    fn attribute_value(&self, _name: &str) -> Option<AttributeValue> {
        self.0
            .as_ref()
            .map(|value| Some(Cow::Owned(value.to_string())))
    }
}

impl<T: Display> RenderOption for &Render<'_, &Option<T>> {
    fn render(&self, escape: bool, out: &mut Vec<Node>) {
        text(
            self.0.as_ref().map(|value| value as &dyn Display),
            escape,
            out,
        )
    }

    fn attribute_value(&self, _name: &str) -> Option<AttributeValue> {
        self.0
            .as_ref()
            .map(|value| Some(Cow::Owned(value.to_string())))
    }
}
