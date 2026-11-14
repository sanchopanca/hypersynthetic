use hypersynthetic::prelude::*;

#[component]
fn Field(r#type: &str, #[builder(default)] title: &str) -> HtmlFragment {
    html! { <input type={r#type} title={title} /> }
}

fn main() {
    // The parameter is written `r#type`, the prop `type`
    let _ = html! { <Field r#type="email" /> };
    // ... also where the name isn't a keyword
    let _ = html! { <Field type="email" r#title="Address" /> };
    // A suggested name leaves the `r#` out too
    let _ = html! { <Field data-r#type="email" /> };
    // Not a name with or without it. (The first error is the compiler's own.)
    let _ = html! { <Field r#self="email" /> };
}
