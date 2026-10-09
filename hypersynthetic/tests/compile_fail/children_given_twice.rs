use hypersynthetic::prelude::*;

#[component]
fn Wrapper(content: HtmlFragment) -> HtmlFragment {
    html! { <div>{{ content }}</div> }
}

fn main() {
    let _ = html! { <Wrapper children={html! {}}><b>"x"</b></Wrapper> };
}
