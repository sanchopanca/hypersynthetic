//! On HTML boolean attributes (`disabled`, `checked`, ...) a `bool` value means the
//! attribute is present or absent, since `disabled="false"` would still disable.
//! On other attributes a `bool` is rendered as `"true"` or `"false"`.

use hypersynthetic::html;

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
