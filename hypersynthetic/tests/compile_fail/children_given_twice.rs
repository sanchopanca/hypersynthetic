use hypersynthetic::prelude::*;

#[component]
fn Wrapper(children: HtmlFragment) -> HtmlFragment {
    html! { <div>{{ children }}</div> }
}

fn main() {
    let _ = html! { <Wrapper children={html! {}}><b>"x"</b></Wrapper> };
}
