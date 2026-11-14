use hypersynthetic::prelude::*;

#[component]
fn Text(name: &str) -> String {
    format!("Hello {name}")
}

// `#[component]` knows a `Result` by its name. Behind an alias it's the compiler
// that finds the wrong type
type Fallible = Result<HtmlFragment, String>;

#[component]
fn Aliased(name: &str) -> Fallible {
    Ok(html! { <p>{name}</p> })
}

fn main() {}
