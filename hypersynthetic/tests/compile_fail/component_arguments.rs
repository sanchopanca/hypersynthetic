use hypersynthetic::prelude::*;

#[component(inline)]
fn Card() -> HtmlFragment {
    html! { <div></div> }
}

fn main() {
    let _ = html! { <Card /> };
}
