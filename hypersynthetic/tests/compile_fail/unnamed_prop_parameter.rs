use hypersynthetic::prelude::*;

#[component]
fn Point((x, y): (i32, i32)) -> HtmlFragment {
    html! { <p>{x}","{y}</p> }
}

fn main() {}
