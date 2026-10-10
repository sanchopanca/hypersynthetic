// `{option}` renders the value inside `Some` like `{value}` would, and nothing for `None`.
use std::fmt::Display;

use hypersynthetic::prelude::*;

#[test]
fn test_option_of_display() {
    let some: Option<&str> = Some("<b>");
    let none: Option<&str> = None;
    let result = html! {
        <p>{some}{none}</p>
        <p>{{ some }}{{ none }}</p>
    };

    assert_eq!(result.to_string(), "<p>&lt;b&gt;</p><p><b></p>");
}

#[test]
fn test_option_of_fragment() {
    let some = Some(html! { <b>"bold"</b> });
    let none: Option<HtmlFragment> = None;
    let result = html! {
        <div>{some}{none}</div>
    };

    assert_eq!(result.to_string(), "<div><b>bold</b></div>");
    let div = result.iter_elements().next().unwrap();
    assert_eq!(div.children.iter_elements().next().unwrap().tag_name, "b");
}

#[test]
fn test_option_references() {
    let text = Some(String::from("text"));
    let frag = Some(html! { <i>"frag"</i> });
    let result = html! {
        <p>{&text}{&frag}{frag.as_ref()}</p>
    };

    assert_eq!(result.to_string(), "<p>text<i>frag</i><i>frag</i></p>");
}

#[test]
fn test_options_are_borrowed_not_moved() {
    let text = Some(String::from("text"));
    let frag = Some(html! { <i>"frag"</i> });
    let result = html! {
        <p>{text}{frag}</p>
    };

    assert_eq!(result.to_string(), "<p>text<i>frag</i></p>");
    assert_eq!(text.as_deref(), Some("text"));
    assert!(frag.is_some());
}

fn generic<T: Display>(value: Option<T>) -> HtmlFragment {
    html! { <p>{value}</p> }
}

#[test]
fn test_option_in_generic_code() {
    assert_eq!(generic(Some(42)).to_string(), "<p>42</p>");
    assert_eq!(generic(None::<i32>).to_string(), "<p></p>");
}

#[component]
fn Greeting(name: Option<&str>) -> HtmlFragment {
    html! { <p>"Hello"{name.map(|_| ", ")}{name}</p> }
}

#[test]
fn test_option_prop() {
    let result = html! {
        <Greeting name={Some("Ann")} />
        <Greeting name={None} />
    };

    assert_eq!(result.to_string(), "<p>Hello, Ann</p><p>Hello</p>");
}

// In an attribute value, `None` leaves the attribute out and `Some` renders the value.

#[test]
fn test_option_attribute_values() {
    let title: Option<&str> = None;
    let id: Option<u32> = Some(7);
    let result = html! {
        <div title={title} id={id} class="c"></div>
    };

    assert_eq!(result.to_string(), r#"<div id="7" class="c"></div>"#);
}

#[test]
fn test_option_for_boolean_attributes() {
    let render = |disabled: bool| html! { <button disabled={disabled.then_some("")}></button> };

    assert_eq!(render(true).to_string(), r#"<button disabled=""></button>"#);
    assert_eq!(render(false).to_string(), "<button></button>");
}

#[test]
fn test_option_attribute_values_are_borrowed_not_moved() {
    let title = Some(String::from("t"));
    let result = html! {
        <a title={title}></a>
        <a title={&title}></a>
    };

    assert_eq!(result.to_string(), r#"<a title="t"></a><a title="t"></a>"#);
    assert_eq!(title.as_deref(), Some("t"));
}

#[test]
fn test_option_value_with_dynamic_attribute_name() {
    let name = "hx-get";
    let url: Option<&str> = None;
    let result = html! {
        <button {name}={url} {name}={Some("/items")}></button>
    };

    assert_eq!(result.to_string(), r#"<button hx-get="/items"></button>"#);
}
