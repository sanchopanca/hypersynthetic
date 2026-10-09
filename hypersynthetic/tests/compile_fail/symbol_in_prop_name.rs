use hypersynthetic::prelude::*;

#[component]
fn Card(click: &str) -> HtmlFragment {
    html! { <div x-on:click={click}></div> }
}

fn main() {
    let _ = html! { <Card @click="go()" /> };
    let _ = html! { <Card x-on:click.prevent="go()" /> };
}
