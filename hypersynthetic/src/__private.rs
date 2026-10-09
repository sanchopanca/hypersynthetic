//! Support code for the macros, not public API.
//!
//! `{expr}` in `html!` inserts `expr` as nodes when it's an [HtmlFragment], and formats
//! it with [Display] otherwise. Trait impls can't express that on stable Rust, since
//! `HtmlFragment` implements `Display` too, so the macro uses autoref specialization:
//! it generates `(&Render(&expr)).render(escape, &mut nodes)`. Method lookup tries the receiver
//! `&Render<T>` first, which only the [RenderFragment] impls accept, and then
//! `&&Render<T>`, which the [RenderDisplay] and [RenderOption] impls accept.
//!
//! An `Option` renders its value like the value itself would, and nothing for `None`.
//! Options of fragments need their own impls next to the fragment ones: a generic
//! `Option<T: Display>` impl would format a fragment instead of inserting it.
//!
//! `Render` holds a reference, so expressions are borrowed like `format!` borrows them.

use std::fmt::Display;

use crate::{HtmlFragment, Node, escape_text};

pub struct Render<'a, T: ?Sized>(pub &'a T);

fn fragment(fragment: Option<&HtmlFragment>, out: &mut Vec<Node>) {
    if let Some(fragment) = fragment {
        out.extend(fragment.iter().cloned());
    }
}

fn text(value: Option<&dyn Display>, escape: bool, out: &mut Vec<Node>) {
    let Some(value) = value else {
        return;
    };
    let text = value.to_string();
    let text = if escape {
        escape_text(text).into_owned()
    } else {
        text
    };
    out.push(Node::Text(text));
}

pub trait RenderFragment {
    fn render(&self, escape: bool, out: &mut Vec<Node>);
}

impl RenderFragment for Render<'_, HtmlFragment> {
    fn render(&self, _escape: bool, out: &mut Vec<Node>) {
        fragment(Some(self.0), out)
    }
}

impl RenderFragment for Render<'_, &HtmlFragment> {
    fn render(&self, _escape: bool, out: &mut Vec<Node>) {
        fragment(Some(self.0), out)
    }
}

impl RenderFragment for Render<'_, Option<HtmlFragment>> {
    fn render(&self, _escape: bool, out: &mut Vec<Node>) {
        fragment(self.0.as_ref(), out)
    }
}

impl RenderFragment for Render<'_, &Option<HtmlFragment>> {
    fn render(&self, _escape: bool, out: &mut Vec<Node>) {
        fragment(self.0.as_ref(), out)
    }
}

impl RenderFragment for Render<'_, Option<&HtmlFragment>> {
    fn render(&self, _escape: bool, out: &mut Vec<Node>) {
        fragment(*self.0, out)
    }
}

pub trait RenderDisplay {
    fn render(&self, escape: bool, out: &mut Vec<Node>);
}

impl<T: Display + ?Sized> RenderDisplay for &Render<'_, T> {
    fn render(&self, escape: bool, out: &mut Vec<Node>) {
        text(Some(&self.0), escape, out)
    }
}

// A separate trait from RenderDisplay: in one trait, the impls for `T: Display`
// and `Option<T>` would conflict, because std could implement Display for Option.
pub trait RenderOption {
    fn render(&self, escape: bool, out: &mut Vec<Node>);
}

impl<T: Display> RenderOption for &Render<'_, Option<T>> {
    fn render(&self, escape: bool, out: &mut Vec<Node>) {
        text(
            self.0.as_ref().map(|value| value as &dyn Display),
            escape,
            out,
        )
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
}
