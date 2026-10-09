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
