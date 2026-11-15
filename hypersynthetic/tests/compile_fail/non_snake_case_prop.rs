#![deny(non_snake_case)]

use hypersynthetic::prelude::*;

// The attribute covers the function and the props struct. The two errors left are
// about the setter that typed-builder generates for the prop, which no attribute
// on the component reaches.
#[component]
#[allow(non_snake_case)]
fn Heading(Title: &str) -> HtmlFragment {
    html! { <h1>{Title}</h1> }
}

fn main() {
    let _ = html! { <Heading Title="x" /> };
}
