use hypersynthetic::prelude::*;

fn main() {
    let name = "Name";
    // Browsers read `</br>` as another `<br>`, and ignore the others
    let _ = html! { <br></br> };
    let _ = html! { <p>"one"<br>"two"</br></p> };
    let _ = html! { <input type="text">{name}</input> };
    let _ = html! { <p><img src="a.png"></img></p> };
    // Like browsers, whatever the case of the name
    let _ = html! { <bR></bR> };
}
