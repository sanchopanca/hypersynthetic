//! On HTML boolean attributes (`disabled`, `checked`, ...) a `bool` value means the
//! attribute is present or absent, since `disabled="false"` would still disable.
//! On other attributes a `bool` is rendered as `"true"` or `"false"`.

use hypersynthetic::prelude::*;
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

#[test]
fn test_bool_on_boolean_attribute() {
    let render = |busy: bool| html! { <button disabled={busy}>"Save"</button> };

    assert_eq!(render(true).to_string(), "<button disabled>Save</button>");
    assert_eq!(render(false).to_string(), "<button>Save</button>");
}

#[test]
fn test_several_boolean_attributes() {
    let (checked, required) = (true, false);
    let result = html! {
        <input type="checkbox" checked={checked} required={required} name="agree" />
    };

    assert_eq!(
        result.to_string(),
        r#"<input type="checkbox" checked name="agree" />"#
    );
}

#[test]
fn test_bool_on_other_attributes_is_text() {
    let result = html! {
        <div aria-expanded={false} hx-boost={true} draggable={false}></div>
    };

    assert_eq!(
        result.to_string(),
        r#"<div aria-expanded="false" hx-boost="true" draggable="false"></div>"#
    );
}

#[test]
fn test_bool_references() {
    let flags = [true, false];
    let result = html! {
        <option :for={selected in &flags} selected={selected}></option>
    };

    assert_eq!(
        result.to_string(),
        "<option selected></option><option></option>"
    );
}

#[test]
fn test_bool_with_dynamic_attribute_name() {
    let attributes = [
        ("checked", true),
        ("HIDDEN", false),
        ("aria-pressed", false),
    ];
    let result = html! {
        <input :for={(name, value) in attributes} {name}={value} />
    };

    assert_eq!(
        result.to_string(),
        r#"<input checked /><input /><input aria-pressed="false" />"#
    );
}

// An `Option<bool>` is a `bool` that may be missing: `None` leaves the attribute out,
// like any `None`, and `Some(flag)` is rendered like `flag`.

#[test]
fn test_optional_bool_on_boolean_attribute() {
    let render = |disabled: Option<bool>| html! { <input disabled={disabled}> };

    assert_eq!(render(Some(true)).to_string(), "<input disabled>");
    assert_eq!(render(Some(false)).to_string(), "<input>");
    assert_eq!(render(None).to_string(), "<input>");
}

#[test]
fn test_optional_bool_on_other_attributes_has_three_states() {
    let render = |open: Option<bool>| html! { <i aria-expanded={open}></i> };

    assert_eq!(
        render(Some(true)).to_string(),
        r#"<i aria-expanded="true"></i>"#
    );
    assert_eq!(
        render(Some(false)).to_string(),
        r#"<i aria-expanded="false"></i>"#
    );
    assert_eq!(render(None).to_string(), "<i></i>");
}

#[component]
fn Toggle(#[builder(default)] disabled: Option<bool>) -> HtmlFragment {
    html! { <input type="checkbox" disabled={disabled}> }
}

#[test]
fn test_optional_bool_prop_passed_to_a_boolean_attribute() {
    let result = html! {
        <Toggle />
        <Toggle disabled={Some(false)} />
        <Toggle disabled={Some(true)} />
    };

    assert_eq!(
        result.to_string(),
        r#"<input type="checkbox"><input type="checkbox"><input type="checkbox" disabled>"#
    );
}

// A `bool` is a `bool`, however it's wrapped

#[test]
fn test_references_to_bool_references() {
    let (on, off) = (true, false);
    let flags: Vec<&bool> = vec![&on, &off];
    let result = html! {
        <input :for={flag in flags.iter()} disabled={flag}>
    };

    assert_eq!(result.to_string(), "<input disabled><input>");
}

#[test]
fn test_bool_behind_pointers() {
    let mut off = false;
    let cow: Cow<bool> = Cow::Owned(false);
    let result = html! {
        <input disabled={Box::new(false)} hidden={Rc::new(false)} required={Arc::new(false)}
            checked={&mut off} readonly={cow}>
        <input disabled={Box::new(true)} hidden={Rc::new(true)} required={Arc::new(true)}>
    };

    assert_eq!(
        result.to_string(),
        "<input><input disabled hidden required>"
    );
}

#[test]
fn test_bool_behind_several_wrappers() {
    let off = false;
    let optional: Option<bool> = Some(false);
    let boxed: Option<Box<bool>> = Some(Box::new(true));
    let result = html! {
        <input disabled={Some(&off)} hidden={&optional} required={boxed}>
    };

    assert_eq!(result.to_string(), "<input required>");
}

#[test]
fn test_optional_bool_with_dynamic_attribute_name() {
    let name = "disabled";
    let result = html! {
        <input {name}={Some(false)}>
        <input {name}={Some(true)}>
    };

    assert_eq!(result.to_string(), "<input><input disabled>");
}

// As content, a `bool` is text in any wrapping

#[test]
fn test_optional_bool_as_content_is_text() {
    let result = html! { <p>{Some(false)}{Box::new(true)}</p> };

    assert_eq!(result.to_string(), "<p>falsetrue</p>");
}
