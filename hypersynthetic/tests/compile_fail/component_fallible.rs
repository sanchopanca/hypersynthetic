use hypersynthetic::prelude::*;

#[component]
fn Fallible(name: &str) -> Result<HtmlFragment, String> {
    Ok(html! { <p>{name}</p> })
}

#[component]
fn Maybe(name: &str) -> Option<HtmlFragment> {
    Some(html! { <p>{name}</p> })
}

// Found by the name `Result`, whatever the path
#[component]
fn Io(name: &str) -> std::io::Result<HtmlFragment> {
    Ok(html! { <p>{name}</p> })
}

fn main() {}
