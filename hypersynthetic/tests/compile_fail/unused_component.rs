#![deny(dead_code)]

use hypersynthetic::prelude::*;

#[component]
fn WithProps(title: &str, count: u32) -> HtmlFragment {
    html! { <p>{title}" "{count}</p> }
}

#[component]
fn WithoutProps() -> HtmlFragment {
    html! { <hr> }
}

#[component]
fn WithChildren(children: HtmlFragment) -> HtmlFragment {
    html! { <div>{children}</div> }
}

// Used, but only by a component that isn't
#[component]
fn Inner(text: &str) -> HtmlFragment {
    html! { <b>{text}</b> }
}

#[component]
fn Outer() -> HtmlFragment {
    html! { <Inner text="x" /> }
}

fn main() {}
