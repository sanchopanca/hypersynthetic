use hypersynthetic::prelude::*;

#[component]
fn Card(title: &str, #[builder(default)] wide: bool) -> HtmlFragment {
    html! { <div data-wide={wide}>{title}</div> }
}

fn main() {
    let _ = html! { <Card title="a" title="b" /> };
    // Without a value, and whatever kind of value each one has
    let _ = html! { <Card title="a" wide wide /> };
    let _ = html! { <Card title="a" wide :if={true} title={"c"} /> };
    // Reported once, at the first repeat
    let _ = html! { <Card title="a" title="b" title="c" /> };
    // With content between the tags too
    let _ = html! { <Card wide={true} title="a" wide={false}>"x"</Card> };
}
