use hypersynthetic::NodeCollection;
use hypersynthetic::{component, html};
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
fn test_attributes_without_values() {
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
        <input type="ckeckbox" checked />
    };

    let string_representation = result.to_string();

    let expected = "<input type=\"ckeckbox\" checked />";
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
fn test_several_elemnts_without_a_parent() {
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

#[component]
fn Component(val1: &str, val2: i32) -> NodeCollection {
    html! {
        <div>
            <p>{val1}</p>
            <p>{val2 + 1}</p>
        </div>
    }
}

#[test]
fn test_component() {
    let result = html! {
        <Component val1="Hello" val2={41} />
    };

    let string_representation = result.to_string();

    let expected = "<div><p>Hello</p><p>42</p></div>";

    assert_eq!(string_representation, expected);
}

#[test]
fn test_component_as_a_child() {
    let result = html! {
        <div>
            <Component val1="test" val2={-1} />
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><div><p>test</p><p>0</p></div></div>";

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

    let s = S { a: "test", b: 42 };
    let result = html! {
        <div id="x{s.a}-{s.b}{s.a}">"Text"</div>
    };

    let string_representation = result.to_string();

    let expected = "<div id=\"xtest-42test\">Text</div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for() {
    let numbers = [1, 2, 3];
    let result = html! {
        <div>
            <p :for={number in numbers}>
                { number }
            </p>
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><p>1</p><p>2</p><p>3</p></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_deep() {
    let things = [(1, 2), (3, 4), (5, 6)];
    let result = html! {
        <tr :for={thing in things}>
            <td>{ thing.0 }</td>
            <td>{ thing.1 }</td>
        </tr>
    };

    let string_representation = result.to_string();

    let expected =
        "<tr><td>1</td><td>2</td></tr><tr><td>3</td><td>4</td></tr><tr><td>5</td><td>6</td></tr>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_selfclosing() {
    let numbers = [1, 2, 3];
    let result = html! {
        <input type="text" :for={number in numbers} id="id{number}" />
    };

    let string_representation = result.to_string();

    let expected = "<input type=\"text\" id=\"id1\" /><input type=\"text\" id=\"id2\" /><input type=\"text\" id=\"id3\" />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_deep_components() {
    let numbers = [1, 2];
    let result = html! {
        <div :for={number in numbers} id="id{number}">
            <Component val1="test" val2={number} />
        </div>
    };

    let string_representation = result.to_string();

    let expected =
        "<div id=\"id1\"><div><p>test</p><p>2</p></div></div><div id=\"id2\"><div><p>test</p><p>3</p></div></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_on_a_component() {
    let numbers = [1, 2];
    let result = html! {
        <Component :for={number in numbers} val1="test" val2={number} />
    };

    let string_representation = result.to_string();

    let expected = "<div><p>test</p><p>2</p></div><div><p>test</p><p>3</p></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_parentheses() {
    let numbers = [(1, "one"), (2, "two)")];
    let result = html! {
        <div :for={(number, name) in numbers}>
            <p>{ number }</p>
            <p>{ name }</p>
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><p>1</p><p>one</p></div><div><p>2</p><p>two)</p></div>";
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
