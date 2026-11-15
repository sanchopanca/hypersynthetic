//! Lint attributes on a component: one attribute covers everything `#[component]`
//! makes of the function. The lints are errors in this file, so a warning that an
//! attribute should have silenced fails the build.
#![deny(dead_code, unused_variables, unfulfilled_lint_expectations)]

use hypersynthetic::prelude::*;

#[component]
#[allow(dead_code)]
fn AllowedUnused(title: &str, count: u32) -> HtmlFragment {
    html! { <p>{title}" "{count}</p> }
}

#[component]
#[expect(dead_code)]
fn ExpectedUnused(title: &str) -> HtmlFragment {
    html! { <p>{title}</p> }
}

#[component]
#[expect(dead_code)]
fn ExpectedUnusedWithoutProps() -> HtmlFragment {
    html! { <hr> }
}

// The attribute before `#[component]`, and with a reason
#[expect(dead_code, reason = "kept for the next release")]
#[component]
fn ExpectedUnusedWithChildren(children: HtmlFragment) -> HtmlFragment {
    html! { <div>{children}</div> }
}

#[component]
#[expect(unused_variables)]
fn IgnoresItsProp(text: &str) -> HtmlFragment {
    html! { <p>"fixed"</p> }
}

#[component]
#[allow(unused_variables)]
fn IgnoresItsProps(first: &str, second: u8) -> HtmlFragment {
    html! { <p>"fixed too"</p> }
}

#[test]
fn test_lint_attributes_reach_the_body() {
    let result = html! {
        <IgnoresItsProp text="unused" />
        <IgnoresItsProps first="unused" second={2} />
    };

    assert_eq!(result.to_string(), "<p>fixed</p><p>fixed too</p>");
}

// A used component isn't dead code, so nothing is expected of it
#[component]
fn Used(text: &str) -> HtmlFragment {
    html! { <b>{text}</b> }
}

#[test]
fn test_a_used_component_has_no_warnings() {
    let result = html! { <Used text="x" /> };

    assert_eq!(result.to_string(), "<b>x</b>");
}
