use hypersynthetic::prelude::*;

fn main() {
    let count = 5;
    let color = "red";
    let _ = html! { <script>{count}</script> };
    let _ = html! { <style>"p {{ color: {color}; }}"</style> };
    // Tag names ignore case: these are a script and a style too
    let _ = html! { <sCRIPT>"let n = "{count}";"</sCRIPT> };
    let _ = html! { <sTYLE>"p {{ color: {color}; }}"</sTYLE> };
}
