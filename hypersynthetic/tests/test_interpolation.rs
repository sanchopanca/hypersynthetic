// `{...}` inside string literals works the same in text nodes and attribute values:
// any expression, optionally followed by a `format!` spec.
use hypersynthetic::prelude::*;

struct Item {
    name: &'static str,
    price: f64,
}

const ITEM: Item = Item {
    name: "tea",
    price: 1.23456,
};

#[test]
fn test_expressions_in_text() {
    let result = html! {
        <p>"{ITEM.name} has {ITEM.name.len()} letters"</p>
    };

    assert_eq!(result.to_string(), "<p>tea has 3 letters</p>");
}

#[test]
fn test_format_spec_in_attribute() {
    let price = ITEM.price;
    let result = html! {
        <data value="{price:.2}"></data>
    };

    assert_eq!(result.to_string(), "<data value=\"1.23\"></data>");
}

#[test]
fn test_expressions_with_format_spec() {
    let result = html! {
        <data value="{ITEM.price:.1}">"{ITEM.price:.3}"</data>
    };

    assert_eq!(result.to_string(), "<data value=\"1.2\">1.235</data>");
}

#[test]
fn test_path_expression_with_format_spec() {
    let result = html! {
        <data value="{std::f64::consts::PI:.2}">"{std::f64::consts::E:.2}"</data>
    };

    assert_eq!(result.to_string(), "<data value=\"3.14\">2.72</data>");
}

#[test]
fn test_debug_format_is_escaped() {
    let name = ITEM.name;
    let result = html! {
        <p title="{name:?}">"{name:?}"</p>
    };

    assert_eq!(
        result.to_string(),
        "<p title=\"&quot;tea&quot;\">\"tea\"</p>"
    );
}

#[test]
fn test_width_from_variable() {
    let n = 7;
    let width = 3;
    let result = html! {
        <pre data-n="[{n:>width$}]">"[{n:<width$}]"</pre>
    };

    assert_eq!(result.to_string(), "<pre data-n=\"[  7]\">[7  ]</pre>");
}

#[test]
fn test_format_spec_variants() {
    const N: (i32, f64) = (255, 1.23456);
    let result = html! {
        <p>"{N.0:x}|{N.0:#X}|{N.0:b}|{N.0:+}|{N.0:*^7}|{N.0: >5}|{N.1:08.3}|{N.0:?}"</p>
    };

    assert_eq!(
        result.to_string(),
        "<p>ff|0xFF|11111111|+255|**255**|  255|0001.235|255</p>"
    );
}
