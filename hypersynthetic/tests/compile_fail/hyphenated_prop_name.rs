use hypersynthetic::prelude::*;

#[component]
fn Card(data_id: &str) -> HtmlFragment {
    html! { <div data-id={data_id}></div> }
}

fn main() {
    let _ = html! { <Card data-id="1" /> };
}
