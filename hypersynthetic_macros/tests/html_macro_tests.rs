use hypersynthetic_macros::html;

#[test]
fn test_tags_and_literal_strings() {
    let result = html! {
        <body>
            <div>
                <p>"Text"</p>
            </div>
        </body>
    };

    let string_representation = result.to_html();

    let expected = "<body><div><p>Text</p></div></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_tags_and_attributes() {
    let result = html! {
        <body id="main" class="container">
            <div>
                <a href="https://example.com">"Link"</a>
            </div>
        </body>
    };

    let string_representation = result.to_html();

    let expected =
        "<body id=\"main\" class=\"container\"><div><a href=\"https://example.com\">Link</a></div></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_several_children() {
    let result = html! {
        <body>
            <div>
                <p>"Text 1"</p>
                <p>"Text 2"</p>
            </div>
        </body>
    };

    let string_representation = result.to_html();

    let expected = "<body><div><p>Text 1</p><p>Text 2</p></div></body>";
    assert_eq!(string_representation, expected);
}
#[test]
fn test_no_children() {
    let result = html! {
        <body>
            <div>
            </div>
        </body>
    };

    let string_representation = result.to_html();

    let expected = "<body><div></div></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_deep_nesting() {
    let result = html! {
        <body>
            <div>
                <p><span><em>"Text"</em></span></p>
            </div>
        </body>
    };

    let string_representation = result.to_html();

    let expected = "<body><div><p><span><em>Text</em></span></p></div></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_one_tag_with_text() {
    let result = html! {
        <p>"Text"</p>
    };

    let string_representation = result.to_html();

    let expected = "<p>Text</p>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_self_closing() {
    let result = html! {
        <body>
            <div>
                <p>"Text 1"</p>
                <br />
                <p>"Text 2"</p>
                <br class="foo" />
                <p>"Text 3"</p>
            </div>
        </body>
    };

    let string_representation = result.to_html();

    let expected =
        "<body><div><p>Text 1</p><br /><p>Text 2</p><br class=\"foo\" /><p>Text 3</p></div></body>";
    assert_eq!(string_representation, expected);
}
