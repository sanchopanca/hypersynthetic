//! Support code for the macros, not public API.
//!
//! `{expr}` in `html!` inserts `expr` as nodes when it's an [HtmlFragment], and formats
//! it with [Display] otherwise. Trait impls can't express that on stable Rust, since
//! `HtmlFragment` implements `Display` too, so the macro uses autoref specialization:
//! it generates `(&Render(&expr)).render_escaped()`. Method lookup tries the receiver
//! `&Render<T>` first, which only the fragment impls accept, and then `&&Render<T>`,
//! which the `Display` impl accepts.
//!
//! `Render` holds a reference, so expressions are borrowed like `format!` borrows them.

use std::fmt::Display;

use crate::{HtmlFragment, Node, escape_text};

pub struct Render<'a, T: ?Sized>(pub &'a T);

pub trait RenderFragment {
    fn render_escaped(&self) -> Vec<Node>;
    fn render_raw(&self) -> Vec<Node>;
}

impl RenderFragment for Render<'_, HtmlFragment> {
    fn render_escaped(&self) -> Vec<Node> {
        self.0.get_nodes()
    }

    fn render_raw(&self) -> Vec<Node> {
        self.0.get_nodes()
    }
}

impl RenderFragment for Render<'_, &HtmlFragment> {
    fn render_escaped(&self) -> Vec<Node> {
        self.0.get_nodes()
    }

    fn render_raw(&self) -> Vec<Node> {
        self.0.get_nodes()
    }
}

pub trait RenderDisplay {
    fn render_escaped(&self) -> Vec<Node>;
    fn render_raw(&self) -> Vec<Node>;
}

impl<T: Display + ?Sized> RenderDisplay for &Render<'_, T> {
    fn render_escaped(&self) -> Vec<Node> {
        vec![Node::Text(escape_text(self.0.to_string()).into_owned())]
    }

    fn render_raw(&self) -> Vec<Node> {
        vec![Node::Text(self.0.to_string())]
    }
}
