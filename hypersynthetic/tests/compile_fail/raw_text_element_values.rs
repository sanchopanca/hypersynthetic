use hypersynthetic::prelude::*;

fn main() {
    let count = 5;
    let color = "red";
    let _ = html! { <script>{count}</script> };
    let _ = html! { <style>"p {{ color: {color}; }}"</style> };
}
