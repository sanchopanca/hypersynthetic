use hypersynthetic::html;
extern crate alloc;

#[test]
fn test_tags_and_literal_strings() {
    let result = html! {
        <body>
            <div>
                <p>"Text"</p>
            </div>
        </body>
    };

    let string_representation = result.to_string();

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

    let string_representation = result.to_string();

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

    let string_representation = result.to_string();

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

    let string_representation = result.to_string();

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

    let string_representation = result.to_string();

    let expected = "<body><div><p><span><em>Text</em></span></p></div></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_one_tag_with_text() {
    let result = html! {
        <p>"Text"</p>
    };

    let string_representation = result.to_string();

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

    let string_representation = result.to_string();

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

    let string_representation = result.to_string();

    let expected = "<body><div><p>text1<em>text1</em>text3</p></div></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_expression() {
    let x = 41;
    let result = html! {
        <p>
            { 1 + x }
        </p>
    };

    let string_representation = result.to_string();

    let expected = "<p>42</p>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_hyphens_in_attribute_names() {
    let result = html! {
        <button hx-get="/resources">"Get 'em"</button>
    };

    let string_representation = result.to_string();

    let expected = "<button hx-get=\"/resources\">Get 'em</button>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_many_hyphens_in_attribute_names() {
    let result = html! {
        <br we-can-have-a-lot-of-hyphens="in the name" />
    };

    let string_representation = result.to_string();

    let expected = "<br we-can-have-a-lot-of-hyphens=\"in the name\" />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_boolean_attribute() {
    let result = html! {
        <input disabled />
    };

    let string_representation = result.to_string();

    let expected = "<input disabled />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attributes_names_which_are_rust_keywords() {
    let result = html! {
        <input type="checkbox" checked />
    };

    let string_representation = result.to_string();

    let expected = "<input type=\"checkbox\" checked />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_boolean_attribute_between_other_attributes() {
    let result = html! {
        <input type="text" required name="text" />
    };

    let string_representation = result.to_string();

    let expected = "<input type=\"text\" required name=\"text\" />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attributes_names_which_are_rust_keywords_with_hyphens() {
    let result = html! {
        <p my-type>"Text"</p>
    };

    let string_representation = result.to_string();

    let expected = "<p my-type>Text</p>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attribute_value_substitution() {
    let x = 3;
    let result = html! {
        <p class={format!("y{x}")}>"text"</p>
    };

    let string_representation = result.to_string();

    let expected = "<p class=\"y3\">text</p>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_attribute_name_substitution() {
    let hx_method = "hx-get";
    let result = html! {
        <button {hx_method}="/resources">"Get 'em"</button>
    };

    let string_representation = result.to_string();

    let expected = "<button hx-get=\"/resources\">Get 'em</button>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_several_elements_without_a_parent() {
    let result = html! {
        <head></head>
        <body></body>
    };

    let string_representation = result.to_string();

    let expected = "<head></head><body></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_doctype() {
    let result = html! {
        <!doctype html>
        <head></head>
        <body></body>
    };

    let string_representation = result.to_string();

    let expected = "<!DOCTYPE html><head></head><body></body>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_escaping_in_expression() {
    let result = html! {
        <div>
            <p>{ "<script>alert(1)</script>" }</p>
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><p>&lt;script&gt;alert(1)&lt;/script&gt;</p></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_escaping_in_literal() {
    let result = html! {
        <div>
            <p>"<script>alert(1)</script>"</p>
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><p>&lt;script&gt;alert(1)&lt;/script&gt;</p></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_escaping_in_attribute_value_literal() {
    let result = html! {
        <div class="<script>alert(1)</script>"></div>
    };

    let string_representation = result.to_string();

    let expected = "<div class=\"&lt;script&gt;alert(1)&lt;/script&gt;\"></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_quote_scaping_in_attribute_value() {
    let value = "\"";
    let result = html! {
        <input value="{value}" />
    };

    let string_representation = result.to_string();

    let expected = "<input value=\"&quot;\" />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_escaping_in_attribute_value_expression() {
    let result = html! {
        <div class={ "<script>alert(1)</script>" }></div>
    };

    let string_representation = result.to_string();

    let expected = "<div class=\"&lt;script&gt;alert(1)&lt;/script&gt;\"></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_escaping_in_attribute_name() {
    let result = html! {
        <div { "<script>alert(1)</script>" }="whatever"></div>
    };

    let string_representation = result.to_string();

    let expected = "<div &lt;script&gt;alert(1)&lt;/script&gt;=\"whatever\"></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_interpolation_in_attr_values() {
    struct S {
        a: &'static str,
        b: i32,
    }

    let s = S { a: "est", b: 42 };
    let result = html! {
        <div id="t{s.a}-{s.b}{s.a}">"Text"</div>
    };

    let string_representation = result.to_string();

    let expected = "<div id=\"test-42est\">Text</div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_colons_in_attr_names() {
    let result = html! {
        <form
            hx-on::after-request="this.reset()"
        >
        </form>
    };

    let string_representation = result.to_string();

    let expected = "<form hx-on::after-request=\"this.reset()\"></form>";
    assert_eq!(string_representation, expected);
}

#[test]
fn document_that_whitespace_in_attribute_names_is_ignored() {
    let result = html! {
        <div data - test = "1"></div>
    };

    let string_representation = result.to_string();

    let expected = "<div data-test=\"1\"></div>";

    assert_eq!(string_representation, expected);
}

#[test]
fn test_disable_html_escaping() {
    let i_know_what_i_am_doing = "<span>I know what I am doing</span>";

    let result = html! {
        <div>{{i_know_what_i_am_doing}}</div>
    };

    let string_representation = result.to_string();

    let expected = "<div><span>I know what I am doing</span></div>";

    assert_eq!(string_representation, expected);
}
