use hypersynthetic::prelude::*;

fn main() {
    let _ = html! { <div title="{oops"></div> };
    let _ = html! { <div title="oops}"></div> };
    let _ = html! { <div x-data="{ open: false }"></div> };
}
