use hypersynthetic::prelude::*;

#[test]
fn test_bare_text() {
    let div = html! {
        <p>Hello world</p>
    };
    assert_eq!(div.to_string(), "<p>Hello world</p>");
}

#[test]
fn test_bare_text_punctuation() {
    let div = html! {
        <p>Hello, world! Can you read this? Crazy; {" isn't "} it?</p>
    };
    assert_eq!(
        div.to_string(),
        "<p>Hello, world! Can you read this? Crazy; isn't it?</p>"
    );
}
