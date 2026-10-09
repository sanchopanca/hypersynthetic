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
