use hypersynthetic::prelude::*;

#[component]
fn Card(id: &str) -> HtmlFragment {
    html! { <div id={id}></div> }
}

fn main() {
    let name = "id";
    let _ = html! { <Card {name}="1" /> };
}
