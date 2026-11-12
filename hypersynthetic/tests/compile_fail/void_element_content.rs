use hypersynthetic::prelude::*;

#[component]
fn Label() -> HtmlFragment {
    html! { "Name" }
}

fn main() {
    let name = "Name";
    let _ = html! { <br>"text"</br> };
    let _ = html! { <input type="text">{name}</input> };
    let _ = html! { <img src="a.png"><b>"bold"</b></img> };
    let _ = html! { <hr><Label /></hr> };
    // Forgetting the `/`: what comes next is read as the element's content
    let _ = html! { <p>"one"<br>"two"</p> };
    // Like browsers, whatever the case of the name
    let _ = html! { <bR>"text"</bR> };
}
