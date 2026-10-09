#![deny(deprecated)]

use hypersynthetic::prelude::*;

#[component]
#[deprecated = "use NewCard"]
fn OldCard() -> HtmlFragment {
    html! { <div></div> }
}

fn main() {
    let _ = html! { <OldCard /> };
}
