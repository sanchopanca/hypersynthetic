use hypersynthetic::prelude::*;

#[test]
fn test_required_textarea() {
    let result = html! {
        <textarea required></textarea>
    };

    assert_eq!(result.to_string(), "<textarea required></textarea>");
}

#[test]
fn test_user_variable_named_v() {
    let v = "hi";
    let result = html! {
        <div>{v}</div>
    };

    assert_eq!(result.to_string(), "<div>hi</div>");
}

#[test]
fn test_user_variable_named_v_in_attribute() {
    let v = "hi";
    let result = html! {
        <div title={v}></div>
    };

    assert_eq!(result.to_string(), r#"<div title="hi"></div>"#);
}

#[test]
fn test_user_variable_named_for_v() {
    let for_v = ["a", "b"];
    let result = html! {
        <li :for={x in for_v}>{x}</li>
    };

    assert_eq!(result.to_string(), "<li>a</li><li>b</li>");
}

#[component]
fn Item(name: String) -> HtmlFragment {
    html! { <li>{name}</li> }
}

#[test]
fn test_user_variable_named_for_v_in_component_loop() {
    let for_v = ["a", "b"];
    let result = html! {
        <Item :for={x in for_v} name={x.to_string()} />
    };

    assert_eq!(result.to_string(), "<li>a</li><li>b</li>");
}

#[test]
fn test_literal_text_between_interpolations_in_attr_value() {
    let a = 1;
    let b = 2;
    let result = html! {
        <div id="{a}xyz{b}"></div>
    };

    assert_eq!(result.to_string(), "<div id=\"1xyz2\"></div>");
}
