use hypersynthetic::prelude::*;

#[test]
fn test_formatting_in_text_node() {
    let text = "some text";
    let result = html! {
        <p>
            "Saying {text}"
        </p>
    };

    let string_representation = result.to_string();

    let expected = "<p>Saying some text</p>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_formatting_in_text_node_disabling_escaping_not_working() {
    #[allow(unused_variables)]
    let text = "<span>some text</span>";
    let result = html! {
        <p>
            "Saying {{text}}"
        </p>
    };

    let string_representation = result.to_string();

    let expected = "<p>Saying {text}</p>";

    assert_eq!(string_representation, expected);
}
