//! Components used by hypersynthetic's cross-crate tests.
//!
//! It's also the library that clippy checks components in: what `#[component]`
//! generates shouldn't be reported for lints about the code a user wrote. These are
//! the ones from clippy's `pedantic` group that it once was.
#![deny(missing_docs)]
#![deny(
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::elidable_lifetime_names
)]

use hypersynthetic::prelude::*;

/// A card with a fixed title and text.
#[component]
#[must_use]
pub fn Card() -> HtmlFragment {
    html! {
        <div class="card">
            <h2>"Card Title"</h2>
            <p>"This is a card component"</p>
        </div>
    }
}

/// A heading with the given text.
#[component]
#[must_use]
pub fn Heading(text: &str) -> HtmlFragment {
    html! { <h1>{text}</h1> }
}

/// A box around its children.
// `html!` reads a value in `{...}` without moving it, and clippy used to say that
// `children` could be a reference. It can't: the content between the tags is passed
// by value.
#[component]
#[must_use]
pub fn Boxed(children: HtmlFragment) -> HtmlFragment {
    html! { <div class="box">{{ children }}</div> }
}

/// A frame around its children, with a caption.
#[component]
#[must_use]
pub fn Framed(caption: &str, children: HtmlFragment) -> HtmlFragment {
    html! { <figure>{children}<figcaption>{caption}</figcaption></figure> }
}

/// A counter whose count is optional.
#[component]
#[must_use]
pub fn Counter(#[builder(default)] count: u32) -> HtmlFragment {
    html! { <span>{count}</span> }
}

/// Two texts that live equally long: the lifetime can't be left out here, though it
/// could in what `#[component]` makes of the function.
#[component]
#[must_use]
pub fn Pair<'x>(first: &'x str, second: &'x str) -> HtmlFragment {
    let longer: &'x str = if first.len() > second.len() {
        first
    } else {
        second
    };
    html! { <span>{first}" "{second}</span><b>{longer}</b> }
}
