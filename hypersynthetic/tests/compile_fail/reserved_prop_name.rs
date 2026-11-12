use hypersynthetic::prelude::*;

#[component]
fn Card(title: &str) -> HtmlFragment {
    html! { <div>{title}</div> }
}

fn main() {
    let _ = html! { <Card self="x" /> };
    let _ = html! { <Card Self="x" /> };
    let _ = html! { <Card super="x" /> };
    let _ = html! { <Card crate="x" /> };
    let _ = html! { <Card _="x" /> };
}
