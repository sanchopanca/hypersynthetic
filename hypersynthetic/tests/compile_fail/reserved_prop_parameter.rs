use hypersynthetic::prelude::*;

#[component]
fn Job(__hs_build: &str) -> HtmlFragment {
    html! { <p>{__hs_build}</p> }
}

#[component]
fn Raw(r#__hs_build: &str) -> HtmlFragment {
    html! { <p>{__hs_build}</p> }
}

fn main() {}
