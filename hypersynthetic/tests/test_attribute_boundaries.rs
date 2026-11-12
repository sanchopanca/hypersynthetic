//! Where one attribute name ends and the next begins. Names can contain `:`, `@`, `.`
//! and `-`, and can start with them too (`:if`, `@click`, `:class`, `.prop`), so only
//! whitespace tells `x-on:click` from `x-data :class`.

use hypersynthetic::prelude::*;

#[test]
fn test_if_after_an_attribute_without_a_value() {
    let show = false;
    let result = html! {
        <button disabled :if={show}>"x"</button>
    };
    assert_eq!(result.to_string(), "");

    let show = true;
    let result = html! {
        <button disabled :if={show}>"x"</button>
    };
    assert_eq!(result.to_string(), "<button disabled>x</button>");
}

#[test]
fn test_if_on_the_next_line_after_an_attribute_without_a_value() {
    let show = true;
    let result = html! {
        <input
            required
            :if={show}
        />
    };
    assert_eq!(result.to_string(), "<input required />");
}

#[test]
fn test_at_sign_starts_a_new_attribute() {
    let result = html! {
        <button disabled @click="go()">"x"</button>
    };
    assert_eq!(
        result.to_string(),
        "<button disabled @click=\"go()\">x</button>"
    );
}

#[test]
fn test_colon_after_whitespace_starts_a_new_attribute() {
    let result = html! {
        <div x-data :class="cls"></div>
    };
    assert_eq!(result.to_string(), "<div x-data :class=\"cls\"></div>");
}

#[test]
fn test_dot_after_whitespace_starts_a_new_attribute() {
    let result = html! {
        <div hidden .prop="1"></div>
    };
    assert_eq!(result.to_string(), "<div hidden .prop=\"1\"></div>");
}

#[test]
fn test_punctuation_without_whitespace_stays_in_the_name() {
    let result = html! {
        <form x-data x-on:submit.prevent="save()" hx-on::after-request="reset()" data-2x></form>
    };
    assert_eq!(
        result.to_string(),
        "<form x-data x-on:submit.prevent=\"save()\" hx-on::after-request=\"reset()\" data-2x></form>"
    );
}

#[test]
fn test_for_after_an_attribute_without_a_value() {
    let result = html! {
        <li hidden :for={n in [1, 2]}>{n}</li>
    };
    assert_eq!(result.to_string(), "<li hidden>1</li><li hidden>2</li>");
}

#[component]
fn Flag(#[builder(default)] on: bool) -> HtmlFragment {
    html! { <i>{on}</i> }
}

#[test]
fn test_if_after_a_prop_without_a_value() {
    let show = true;
    let result = html! {
        <Flag on :if={show} />
        <Flag on :if={!show} />
    };
    assert_eq!(result.to_string(), "<i>true</i>");
}

#[test]
fn test_comment_between_attributes_counts_as_whitespace() {
    let result = html! {
        <div hidden/* why */:class="c"></div>
    };
    assert_eq!(result.to_string(), "<div hidden :class=\"c\"></div>");
}

// In a `macro_rules!` macro, the parts of a name can be written in different places:
// here the words come from the call and the punctuation from the macro. Their
// positions say nothing about whitespace, so the name stays in one piece, and `:if`
// is still recognised.
macro_rules! field {
    ($prefix:ident, $suffix:ident, $flag:ident, $show:expr) => {
        html! { <input $prefix-$suffix="v" $prefix:$suffix="w" $flag :if={$show} /> }
    };
}

#[test]
fn test_names_put_together_by_macro_rules() {
    assert_eq!(
        field!(data, id, required, true).to_string(),
        "<input data-id=\"v\" data:id=\"w\" required />"
    );
    assert_eq!(field!(data, id, required, false).to_string(), "");
}
