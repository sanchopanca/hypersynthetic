use hypersynthetic::prelude::*;

#[component]
fn Card(title: &str) -> HtmlFragment {
    html! { <h1>{title}</h1> }
}

fn main() {
    let _ = html! { <Card title="x"><p>"child"</p></Card> };
}
