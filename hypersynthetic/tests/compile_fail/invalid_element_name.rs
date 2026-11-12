use hypersynthetic::prelude::*;

fn main() {
    // A tag name starts with a letter from a to z. Browsers don't read these as tags
    // at all: `<_foo>` is shown as text.
    let _ = html! { <_foo>"text"</_foo> };
    let _ = html! { <_ /> };
    let _ = html! { <über>"text"</über> };
    let _ = html! { <p><_my-widget>"text"</_my-widget></p> };
    // Not a component either: its name would start with an uppercase letter
    let _ = html! { <_Card /> };
}
