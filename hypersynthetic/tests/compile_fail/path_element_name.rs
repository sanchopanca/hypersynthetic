use hypersynthetic::prelude::*;

fn main() {
    let _ = html! { <foo::bar></foo::bar> };
    let _ = html! { <::div></::div> };
    let _ = html! { <p><::div></::div></p> };
}
