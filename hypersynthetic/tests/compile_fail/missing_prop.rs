use hypersynthetic::prelude::*;

mod ui {
    use hypersynthetic::prelude::*;

    #[component]
    pub fn Job(title: &str, count: u32) -> HtmlFragment {
        html! { <p>{title}{count}</p> }
    }
}

use ui::Job;

fn main() {
    let n = 1;
    // A missing prop is reported at the tag's name, whatever the other props are
    let _ = html! { <div><Job title="x" /></div> };
    let _ = html! { <div><Job title="x{n}" /></div> };
    let _ = html! { <div><Job count={n} /></div> };
    // At the component's name in a path
    let _ = html! { <ui::Job title="x" /> };
}
