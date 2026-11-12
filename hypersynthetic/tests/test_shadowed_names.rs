//! The code that `html!` and `#[component]` generate is pasted into the caller's scope,
//! where `Some`, `None`, `format!` and friends can mean something else. It must always
//! get the ones from std. See also `test_paths.rs`, about the crate's own name.

// What a caller might have in scope: an enum with `Some` and `None` variants, imported
// with a glob, types named like std's, and a `format!` macro of their own.
mod shadows {
    pub enum Choice {
        Some(u8),
        None,
    }
    pub struct Option;
    pub struct Vec;
    pub struct String;
    pub struct Cow;
}

#[allow(unused_imports)]
use shadows::{Choice::*, *};

#[allow(unused_macros)]
macro_rules! format {
    ($($args:tt)*) => {
        compile_error!("generated code used the `format!` in scope")
    };
}

use hypersynthetic::prelude::*;

#[component]
fn Labelled(text: &str, label: std::string::String) -> HtmlFragment {
    html! { <label title="{label}">{text}</label> }
}

#[test]
fn test_generated_code_ignores_shadowed_std_names() {
    let id = 7;
    let name = "data-name";
    let title: std::option::Option<&str> = std::option::Option::None;
    let on = true;

    let result = html! {
        <p class="literal" hidden id={id} data-x="x{id}" {name}="v" title={title} disabled={on}>
            "text {id}"
            "{id + 1:>3}"
            {id}
            {{ id }}
        </p>
        <Labelled text="a" label="n{id}" />
    };

    assert_eq!(
        result.to_string(),
        "<p class=\"literal\" hidden id=\"7\" data-x=\"x7\" data-name=\"v\" disabled>\
         text 7  877</p>\
         <label title=\"n7\">a</label>"
    );
}

// The shadows are really in scope: these are the caller's, not std's
#[test]
fn test_the_names_are_shadowed() {
    let choices = [Some(1), None];
    assert!(matches!(choices[0], Choice::Some(1)));
    assert!(matches!(choices[1], Choice::None));
    let _ = (Option, Vec, String, Cow);
}
