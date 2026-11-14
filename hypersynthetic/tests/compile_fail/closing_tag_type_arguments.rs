use hypersynthetic::prelude::*;
use std::fmt::Display;

#[component]
fn Frame<T: Display>(label: T, children: HtmlFragment) -> HtmlFragment {
    html! { <fieldset><legend>{label}</legend>{children}</fieldset> }
}

mod ui {
    // `html!` refuses the tags below before the path is looked up
    #[allow(unused_imports)]
    pub(crate) use super::Frame;
}

fn main() {
    // A closing tag can leave the type arguments out, but not get them wrong
    let _ = html! { <Frame::<char> label={'a'}>"x"</Frame::<u8>> };
    // ... with or without the `::`, which the message adds
    let _ = html! { <Frame<char> label={'a'}>"x"</Frame<u8>> };
    // ... or have ones the opening tag doesn't
    let _ = html! { <Frame label={'a'}>"x"</Frame::<char>> };
    // Only the arguments can be left out: the path is still needed
    let _ = html! { <ui::Frame::<char> label={'a'}>"x"</Frame> };
    let _ = html! { <Frame::<char> label={'a'}>"x"</ui::Frame> };
    // ... all of it, also where the part that is there matches
    let _ = html! { <Ui::Frame::<char> label={'a'}>"x"</Ui> };
    let _ = html! { <Frame::<char> label={'a'}>"x"</Frame::Inner> };
    // The name without its arguments is still compared exactly
    let _ = html! { <Frame::<char> label={'a'}>"x"</Shows> };
}
