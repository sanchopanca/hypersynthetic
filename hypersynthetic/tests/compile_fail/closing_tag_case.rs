use hypersynthetic::prelude::*;

#[component]
fn Card(children: HtmlFragment) -> HtmlFragment {
    html! { <div>{children}</div> }
}

#[component]
fn Br(children: HtmlFragment) -> HtmlFragment {
    html! { <div>{children}</div> }
}

fn main() {
    // A name that starts with an uppercase letter is a component, in a closing tag too
    let _ = html! { <div>"x"</DIV> };
    let _ = html! { <div>"x"</Div> };
    // A void element has no closing tag, whatever its case
    let _ = html! { <p>"a"<br></BR></p> };
    let _ = html! { <Card><br></Br></Card> };
    // But a component can have a name like that
    let _ = html! { <Br>"x"</BR> };
    // Component names are Rust paths: their case matters
    let _ = html! { <Card>"x"</card> };
    let _ = html! { <Card>"x"</CARD> };
    // HTML only ignores the case of ASCII letters
    let _ = html! { <café>"x"</cafÉ> };
}
