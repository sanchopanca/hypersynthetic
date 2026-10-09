use hypersynthetic::prelude::*;

fn main() {
    let _ = html! { <p title="{missing}"></p> };
    let _ = html! { <p>"{absent.len()}"</p> };
}
