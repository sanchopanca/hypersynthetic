use test_component_lib::Card;

#[test]
fn test_imported_component() {
    use hypersynthetic::prelude::*;

    let result = html! {
        <Card />
    };

    let string_representation = result.to_string();

    let expected = r#"<div class="card"><h2>Card Title</h2><p>This is a card component</p></div>"#;

    assert_eq!(string_representation, expected);
}

#[test]
fn test_imported_component_with_optional_prop() {
    use hypersynthetic::prelude::*;
    use test_component_lib::Counter;

    let result = html! {
        <Counter />
        <Counter count={2} />
    };

    assert_eq!(result.to_string(), "<span>0</span><span>2</span>");
}

// A path can start with `::`, to name a crate whatever is in scope:
// `<::test_component_lib::Card />`.

#[test]
fn test_absolute_path_as_a_child() {
    use hypersynthetic::prelude::*;

    let result = html! {
        <::test_component_lib::Heading text="top" />
        <div><::test_component_lib::Heading text="child" /></div>
    };

    assert_eq!(result.to_string(), "<h1>top</h1><div><h1>child</h1></div>");
}

#[test]
fn test_absolute_path_with_a_closing_tag() {
    use hypersynthetic::prelude::*;

    let result = html! {
        <::test_component_lib::Boxed>
            "text"
            <::test_component_lib::Heading text="nested" />
        </::test_component_lib::Boxed>
    };

    assert_eq!(
        result.to_string(),
        "<div class=\"box\">text<h1>nested</h1></div>"
    );
}
