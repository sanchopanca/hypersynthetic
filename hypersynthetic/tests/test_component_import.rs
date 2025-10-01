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
