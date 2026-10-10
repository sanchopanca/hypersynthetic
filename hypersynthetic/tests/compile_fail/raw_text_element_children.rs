use hypersynthetic::prelude::*;

#[component]
fn Rule() -> HtmlFragment {
    html! { "p {{ color: red; }}" }
}

fn main() {
    let _ = html! { <script>"let a = 1;"<b>"bold"</b></script> };
    let _ = html! { <style><Rule /></style> };
}
