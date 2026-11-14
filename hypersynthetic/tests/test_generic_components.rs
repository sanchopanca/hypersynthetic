use hypersynthetic::prelude::*;
use std::fmt::Display;

// A generic component is used like a generic function. Where a prop has the type
// `T`, the prop's value says what `T` is. It can also be written out, with the
// turbofish a function call would have: `shows::<u64>(5)` and `<Shows::<u64> />`.
#[component]
fn Shows<T: Display>(value: T) -> HtmlFragment {
    html! { <b>{value}</b> }
}

#[component]
fn Frame<T: Display>(label: T, children: HtmlFragment) -> HtmlFragment {
    html! { <fieldset><legend>{label}</legend>{children}</fieldset> }
}

mod ui {
    pub(crate) use super::{Frame, Shows};
}

#[test]
fn test_type_arguments_in_a_tag() {
    let result = html! {
        <Shows::<u64> value={5} />
        <Shows::<_> value={6u8} />
        <Shows::<Box<&str>> value={Box::new("boxed")} />
        <ui::Shows::<String> value="owned" />
        <p><Shows::<f32> value={1.5} :if={true} /></p>
        <Frame::<char> label={'a'}>"content"</Frame::<char>>
    };

    assert_eq!(
        result.to_string(),
        "<b>5</b><b>6</b><b>boxed</b><b>owned</b><p><b>1.5</b></p>\
         <fieldset><legend>a</legend>content</fieldset>"
    );
}

// A closing tag says which tag it closes, and the name does that: the type
// arguments can be left out, like the props.
#[test]
fn test_closing_tag_without_type_arguments() {
    let result = html! {
        <Frame::<char> label={'a'}>"short"</Frame>
        <Frame::<char> label={'b'}>"long"</Frame::<char>>
        <ui::Frame::<Box<Box<&str>>> label={Box::new(Box::new("c"))} :if={true}>"path"</ui::Frame>
        <Frame::<u8> label={1}>
            <Frame::<u8> label={2}>"nested"</Frame>
        </Frame>
    };

    assert_eq!(
        result.to_string(),
        "<fieldset><legend>a</legend>short</fieldset>\
         <fieldset><legend>b</legend>long</fieldset>\
         <fieldset><legend>c</legend>path</fieldset>\
         <fieldset><legend>1</legend><fieldset><legend>2</legend>nested</fieldset></fieldset>"
    );
}

// In Rust code the `::` keeps `Shows<u64>` from being read as two comparisons. A tag
// can't be read that way, so it can be left out there, in any tag and in any mix.
#[test]
fn test_type_arguments_without_the_path_separator() {
    let result = html! {
        <Shows<u64> value={5} />
        <Shows<_> value={6u8} />
        <Shows<Box<&str>> value={Box::new("boxed")} />
        <ui::Shows<String> value="owned" />
        <p><Shows<f32> value={1.5} :if={true} /></p>
        <Frame<char> label={'a'}>"short"</Frame>
        <Frame<char> label={'b'}>"same"</Frame<char>>
        <Frame<char> label={'c'}>"mixed"</Frame::<char>>
        <Frame::<char> label={'d'}>"mixed"</Frame<char>>
    };

    assert_eq!(
        result.to_string(),
        "<b>5</b><b>6</b><b>boxed</b><b>owned</b><p><b>1.5</b></p>\
         <fieldset><legend>a</legend>short</fieldset>\
         <fieldset><legend>b</legend>same</fieldset>\
         <fieldset><legend>c</legend>mixed</fieldset>\
         <fieldset><legend>d</legend>mixed</fieldset>"
    );
}

// The parameters below aren't the type of any prop, so no value can say what they
// are: the tag has to.

trait Unit {
    const SUFFIX: &'static str;
}

struct Kilograms;
struct Litres;

impl Unit for Kilograms {
    const SUFFIX: &'static str = "kg";
}

impl Unit for Litres {
    const SUFFIX: &'static str = "l";
}

#[component]
fn Amount<U: Unit>(value: f64) -> HtmlFragment {
    html! { <span>{value}" "{U::SUFFIX}</span> }
}

#[component]
fn Zero<T: Default + Display>() -> HtmlFragment {
    html! { <i>{T::default()}</i> }
}

#[test]
fn test_type_parameter_no_prop_uses() {
    let result = html! {
        <Amount::<Kilograms> value={2.5} />
        <Amount::<Litres> value={0.5} />
        <Zero::<u32> />
        <Zero::<bool> />
        <Amount<Litres> value={1.5} />
        <Zero<u8> />
    };

    assert_eq!(
        result.to_string(),
        "<span>2.5 kg</span><span>0.5 l</span><i>0</i><i>false</i>\
         <span>1.5 l</span><i>0</i>"
    );
}

#[component]
fn Named<T: ?Sized>(children: HtmlFragment) -> HtmlFragment {
    html! { <section data-type={std::any::type_name::<T>()}>{children}</section> }
}

#[test]
fn test_unused_type_parameter_with_children() {
    let result = html! {
        <Named::<u8>>
            <Named::<str>>"inside"</Named::<str>>
        </Named::<u8>>
        <Named<i8>><Named<Vec<i8>>>"short"</Named></Named<i8>>
    };

    assert_eq!(
        result.to_string(),
        "<section data-type=\"u8\"><section data-type=\"str\">inside</section></section>\
         <section data-type=\"i8\"><section data-type=\"alloc::vec::Vec&lt;i8&gt;\">short</section></section>"
    );
}

#[component]
fn Described<T>(label: &str) -> HtmlFragment
where
    T: Default + Display,
{
    html! { <p>{label}": "{T::default()}</p> }
}

#[test]
fn test_type_parameter_only_a_where_clause_names() {
    let result = html! { <Described::<i8> label="default" /> };

    assert_eq!(result.to_string(), "<p>default: 0</p>");
}

#[component]
fn Stars<const N: usize>() -> HtmlFragment {
    html! { <span>{"*".repeat(N)}</span> }
}

#[component]
fn Columns<const N: usize>(cells: [&str; N]) -> HtmlFragment {
    html! { <tr data-columns={N}><td :for={cell in cells}>{cell}</td></tr> }
}

#[test]
fn test_const_parameter() {
    let result = html! {
        <Stars::<3> />
        <Stars::<{ 1 + 1 }> />
        <Columns cells={["a", "b"]} />
        <Stars<1> />
        <Stars<{ 2 * 2 }> />
    };

    assert_eq!(
        result.to_string(),
        "<span>***</span><span>**</span><tr data-columns=\"2\"><td>a</td><td>b</td></tr>\
         <span>*</span><span>****</span>"
    );
}

// A lifetime no prop uses has nothing to be told: it's inferred, as in a function call.
// (Clippy says such a lifetime is pointless, here as on any function.)
#[component]
#[allow(clippy::extra_unused_lifetimes)]
fn Idle<'x>() -> HtmlFragment {
    html! { <hr /> }
}

#[component]
#[allow(clippy::extra_unused_lifetimes)]
fn Mixed<'x, T: Display, U: Default + Display, const N: usize>(first: T) -> HtmlFragment {
    html! { <p>{first}" "{U::default()}" "{N}</p> }
}

#[test]
fn test_unused_lifetime_parameter() {
    let result = html! {
        <Idle />
        <Mixed::<&str, u8, 7> first="one" />
        <Mixed::<_, bool, 0> first={1} />
    };

    assert_eq!(result.to_string(), "<hr /><p>one 0 7</p><p>1 false 0</p>");
}

// `T` is in the type of a prop here, but only as `T::Item`, and many iterators have
// the same `Item`: the value doesn't say which `T` it is.
#[component]
fn Sample<T: Iterator>(item: T::Item) -> HtmlFragment
where
    T::Item: Display,
{
    html! { <samp>{item}</samp> }
}

#[test]
fn test_type_parameter_used_through_an_associated_type() {
    let result = html! {
        <Sample::<std::vec::IntoIter<u8>> item={1} />
        <Sample::<std::str::Chars> item={'c'} />
    };

    assert_eq!(result.to_string(), "<samp>1</samp><samp>c</samp>");
}

// The props struct of a generic component has a field of its own, which no prop's
// name can collide with.
#[component]
fn Marked<T: Display>(__hs_marker: T, r#__hs_marker_: T) -> HtmlFragment {
    html! { <u>{__hs_marker}{__hs_marker_}</u> }
}

#[test]
fn test_prop_named_like_the_marker_field() {
    let result = html! { <Marked __hs_marker="a" __hs_marker_="b" /> };

    assert_eq!(result.to_string(), "<u>ab</u>");
}
