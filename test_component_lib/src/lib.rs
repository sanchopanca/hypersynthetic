//! Components used by hypersynthetic's cross-crate tests.
#![deny(missing_docs)]

use hypersynthetic::prelude::*;

/// A card with a fixed title and text.
#[component]
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
pub fn Heading(text: &str) -> HtmlFragment {
    html! { <h1>{text}</h1> }
}

/// A box around its children.
#[component]
pub fn Boxed(children: HtmlFragment) -> HtmlFragment {
    html! { <div class="box">{{ children }}</div> }
}

/// A counter whose count is optional.
#[component]
pub fn Counter(#[builder(default)] count: u32) -> HtmlFragment {
    html! { <span>{count}</span> }
}
