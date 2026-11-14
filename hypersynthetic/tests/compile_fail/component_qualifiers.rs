use hypersynthetic::prelude::*;

#[component]
async fn Async(name: &str) -> HtmlFragment {
    html! { <p>{name}</p> }
}

#[component]
unsafe fn Unsafe(name: &str) -> HtmlFragment {
    html! { <p>{name}</p> }
}

#[component]
const fn Const(name: &str) -> HtmlFragment {
    html! { <p>{name}</p> }
}

#[component]
extern "C" fn Extern(name: &str) -> HtmlFragment {
    html! { <p>{name}</p> }
}

// Every one of them is reported, not only the first
#[component]
pub async unsafe fn Both(name: &str) -> HtmlFragment {
    html! { <p>{name}</p> }
}

fn main() {}
