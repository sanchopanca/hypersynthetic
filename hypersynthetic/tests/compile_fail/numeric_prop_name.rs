use hypersynthetic::prelude::*;

#[component]
fn Card(title: &str) -> HtmlFragment {
    html! { <div>{title}</div> }
}

fn main() {
    let _ = html! { <Card 5="five" /> };
    let _ = html! { <Card 2x="twice" /> };
    let _ = html! { <Card 2-x="two" /> };
}
