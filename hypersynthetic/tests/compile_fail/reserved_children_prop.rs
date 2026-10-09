use hypersynthetic::prelude::*;

#[component]
fn Panel(content: HtmlFragment, children: &str) -> HtmlFragment {
    html! { <div>{children}{{ content }}</div> }
}

fn main() {}
