use hypersynthetic::prelude::*;

#[component]
fn Text(name: &str) -> String {
    format!("Hello {name}")
}

#[component]
fn Fallible(name: &str) -> Result<HtmlFragment, String> {
    Ok(html! { <p>{name}</p> })
}

fn main() {}
