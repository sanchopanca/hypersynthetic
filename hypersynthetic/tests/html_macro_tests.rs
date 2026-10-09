use hypersynthetic::{ElementData, HtmlFragment, Node, html};
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

    let expected = "<body id=\"main\" class=\"container\"><div><a href=\"https://example.com\">Link</a></div></body>";
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
fn test_self_closing_non_void_element() {
    let result = html! {
        <div class="a" />
        <p>"Text"</p>
    };

    let expected = "<div class=\"a\"></div><p>Text</p>";
    assert_eq!(result.to_string(), expected);
}

#[test]
fn test_self_closing_void_elements() {
    let result = html! {
        <area /><base /><br /><col /><embed /><hr /><img /><input />
        <link /><meta /><source /><track /><wbr />
    };

    let expected = "<area /><base /><br /><col /><embed /><hr /><img /><input />\
                    <link /><meta /><source /><track /><wbr />";
    assert_eq!(result.to_string(), expected);
}

#[test]
fn test_self_closing_non_void_element_built_manually() {
    let mut element = ElementData::new("span".to_owned());
    element.self_closing = true;
    let fragment = HtmlFragment::new(vec![Node::Element(element)]);

    assert_eq!(fragment.to_string(), "<span></span>");
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
fn test_attribute_name_substitution_with_unusual_valid_names() {
    let names = ["@click", "x-on:submit.prevent", "data-ünïcode"];
    let result = html! {
        <button :for={name in names} {name}="v"></button>
    };

    let expected = "<button @click=\"v\"></button>\
                    <button x-on:submit.prevent=\"v\"></button>\
                    <button data-ünïcode=\"v\"></button>";
    assert_eq!(result.to_string(), expected);
}

#[test]
fn test_attribute_name_substitution_cannot_inject_attributes() {
    let name = "x onmouseover=alert(1) y";
    let result = html! {
        <div {name}="v" class="c"></div>
    };

    assert_eq!(result.to_string(), "<div class=\"c\"></div>");
}

#[test]
fn test_attribute_name_substitution_cannot_break_out_of_tag() {
    let names = [
        "x><script>alert(1)</script",
        "x\"",
        "x'",
        "x/",
        "x=",
        "x\ty",
        "x\0",
        "",
    ];
    let result = html! {
        <div :for={name in names} {name}="v" class="c"></div>
    };

    assert_eq!(
        result.to_string(),
        "<div class=\"c\"></div>".repeat(names.len())
    );
}

#[test]
fn test_set_attribute_with_invalid_name_is_not_rendered() {
    let mut element = ElementData::new("div".to_owned());
    element.set_attribute("x onclick".to_owned(), "alert(1)".to_owned());

    assert_eq!(element.to_string(), "<div></div>");
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
fn test_invalid_attribute_name_is_not_rendered() {
    let result = html! {
        <div { "<script>alert(1)</script>" }="whatever"></div>
    };

    let string_representation = result.to_string();

    let expected = "<div></div>";
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
fn test_escaped_braces_in_attr_value() {
    let result = html! {
        <div x-data="{{ open: false }}" hx-vals="{{\"a\": 1}}"></div>
    };

    assert_eq!(
        result.to_string(),
        "<div x-data=\"{ open: false }\" hx-vals=\"{&quot;a&quot;: 1}\"></div>"
    );
}

#[test]
fn test_escaped_braces_and_interpolation_in_attr_value() {
    let count = 5;
    let result = html! {
        <div x-data="{{ count: {count} }}"></div>
    };

    assert_eq!(result.to_string(), "<div x-data=\"{ count: 5 }\"></div>");
}

#[test]
fn test_escaped_braces_are_consistent_between_text_and_attr_value() {
    let result = html! {
        <p title="{{x}}">"{{x}}"</p>
    };

    assert_eq!(result.to_string(), "<p title=\"{x}\">{x}</p>");
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

#[test]
fn test_custom_elements() {
    let result = html! {
        <my-widget some-attr="x">
            <x-child-element />
            <sl-button-group :for={i in 0..2}>{i}</sl-button-group>
        </my-widget>
    };

    assert_eq!(
        result.to_string(),
        "<my-widget some-attr=\"x\"><x-child-element></x-child-element>\
         <sl-button-group>0</sl-button-group><sl-button-group>1</sl-button-group></my-widget>"
    );
}

#[test]
fn test_keyword_tag_names() {
    let result = html! {
        <svg>
            <use href="#icon" />
            <use href="#other"></use>
        </svg>
        <x-type></x-type>
    };

    assert_eq!(
        result.to_string(),
        "<svg><use href=\"#icon\"></use><use href=\"#other\"></use></svg><x-type></x-type>"
    );
}

#[test]
fn test_alpine_attribute_names() {
    let result = html! {
        <form
            @click="open = !open"
            @click.outside="open = false"
            x-on:submit.prevent="save()"
            x-on:input.debounce.500ms="search()"
            :class="{{ active: open }}"
            data-2="two"
        ></form>
    };

    assert_eq!(
        result.to_string(),
        "<form \
         @click=\"open = !open\" \
         @click.outside=\"open = false\" \
         x-on:submit.prevent=\"save()\" \
         x-on:input.debounce.500ms=\"search()\" \
         :class=\"{ active: open }\" \
         data-2=\"two\"\
         ></form>"
    );
}
