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

#[test]
fn test_mixed_children() {
    let result = html! {
        <body>
            <div>
                <p>"text1"<em>"text1"</em>"text3"</p>
            </div>
        </body>
    };

    let string_representation = result.to_html();

    let expected = "<body><div><p>text1<em>text1</em>text3</p></div></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_mixed_attributes() {
    let x = 41;
    let result = html! {
        <p>
            { 1 + x }
        </p>
    };

    let string_representation = result.to_html();

    let expected = "<p>42</p>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_hyphens_in_attribute_names() {
    let result = html! {
        <button hx-get="/resources">"Get 'em"</button>
    };

    let string_representation = result.to_html();

    let expected = "<button hx-get=\"/resources\">Get 'em</button>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_many_hyphens_in_attribute_names() {
    let result = html! {
        <br we-can-have-a-lot-of-hyphens="in the name" />
    };

    let string_representation = result.to_html();

    let expected = "<br we-can-have-a-lot-of-hyphens=\"in the name\" />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attributes_without_values() {
    let result = html! {
        <input disabled />
    };

    let string_representation = result.to_html();

    let expected = "<input disabled />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attributes_names_which_are_rust_keywords() {
    let result = html! {
        <input type="ckeckbox" checked />
    };

    let string_representation = result.to_html();

    let expected = "<input type=\"ckeckbox\" checked />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attributes_names_which_are_rust_keywords_with_hyphens() {
    let result = html! {
        <p my-type>"Text"</p>
    };

    let string_representation = result.to_html();

    let expected = "<p my-type>Text</p>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attribute_value_substitution() {
    let x = 3;
    let result = html! {
        <p class={format!("y{x}")}>"text"</p>
    };

    let string_representation = result.to_html();

    let expected = "<p class=\"y3\">text</p>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attribute_name_substitution() {
    let hx_method = "hx-get";
    let result = html! {
        <button {hx_method}="/resources">"Get 'em"</button>
    };

    let string_representation = result.to_html();

    let expected = "<button hx-get=\"/resources\">Get 'em</button>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_several_elemnts_without_a_parent() {
    let result = html! {
        <head></head>
        <body></body>
    };

    let string_representation = result.to_html();

    let expected = "<head></head><body></body>";
    assert_eq!(string_representation, expected);
}
// TODO: components
// TODO: <!doctype>
// TODO: comments
// TODO: rest of the keywords in attribute names
// TODO: keywords in tag names
// TODO: extend possible attribute names ('-' is not the only valid character that can be used)
