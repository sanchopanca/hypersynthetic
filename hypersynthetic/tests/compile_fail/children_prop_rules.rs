use hypersynthetic::prelude::*;

#[component]
fn List(children: Vec<String>) -> HtmlFragment {
    html! { <li :for={item in children}>{item}</li> }
}

#[component]
fn Ignores(_children: HtmlFragment) -> HtmlFragment {
    html! {}
}

fn main() {}
