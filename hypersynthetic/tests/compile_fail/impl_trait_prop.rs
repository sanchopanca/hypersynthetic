use hypersynthetic::prelude::*;

#[component]
fn Shows(value: impl std::fmt::Display) -> HtmlFragment {
    html! { <p>{value}</p> }
}

#[component]
fn List(items: Vec<impl std::fmt::Display>) -> HtmlFragment {
    html! { <li :for={item in items}>{item}</li> }
}

fn main() {
    let _ = html! {
        <Shows value={1} />
        <List items={vec![1, 2]} />
    };
}
