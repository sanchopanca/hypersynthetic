// A string written in the template, `text="Hello"` or `text="Hello {name}"`, is a
// value for a `&str` prop and for a `String` prop alike.
use hypersynthetic::prelude::*;

#[component]
fn Borrowed(text: &str) -> HtmlFragment {
    html! { <p>{text}</p> }
}

#[component]
fn Owned(text: String) -> HtmlFragment {
    html! { <p>{text}</p> }
}

#[test]
fn test_interpolated_string_for_a_str_prop() {
    let name = "World";
    let result = html! { <Borrowed text="Hello {name}" /> };

    assert_eq!(result.to_string(), "<p>Hello World</p>");
}

#[test]
fn test_plain_string_for_a_string_prop() {
    let result = html! { <Owned text="Hello" /> };

    assert_eq!(result.to_string(), "<p>Hello</p>");
}

#[test]
fn test_the_combinations_that_always_worked() {
    let name = "World";
    let result = html! {
        <Borrowed text="Hello" />
        <Owned text="Hello {name}" />
    };

    assert_eq!(result.to_string(), "<p>Hello</p><p>Hello World</p>");
}

// A setter that is generic doesn't say which of the two it wants, so the string is
// passed as it always was: a `&'static str` when it's written out, a `String` when it
// has `{...}` in it. The bounds here only hold for those types.

#[component]
fn Converted(#[builder(setter(into))] text: String) -> HtmlFragment {
    html! { <p>{text}</p> }
}

#[component]
fn Copied<T: std::fmt::Display + Copy>(text: T) -> HtmlFragment {
    let copy = text;
    html! { <p>{text}{copy}</p> }
}

#[component]
fn Kept<T: std::fmt::Display + 'static>(text: T) -> HtmlFragment {
    html! { <p>{text}</p> }
}

#[test]
fn test_generic_setters_get_the_string_as_written() {
    let name = "World";
    let result = html! {
        <Converted text="Hello" />
        <Converted text="Hello {name}" />
        <Copied text="Hello" />
        <Kept text="Hello" />
        <Kept text="Hello {name}" />
    };

    assert_eq!(
        result.to_string(),
        "<p>Hello</p><p>Hello World</p><p>HelloHello</p><p>Hello</p><p>Hello World</p>"
    );
}

// Other types that hold a string say so with `setter(into)`
#[component]
fn Flexible(
    #[builder(setter(into))] cow: std::borrow::Cow<'_, str>,
    #[builder(setter(into))] boxed: Box<str>,
) -> HtmlFragment {
    html! { <p>{cow}{boxed}</p> }
}

#[test]
fn test_other_string_types_with_setter_into() {
    let name = "World";
    let result = html! {
        <Flexible cow="Hello" boxed=" {name}" />
        <Flexible cow="Hello {name}" boxed="!" />
    };

    assert_eq!(result.to_string(), "<p>Hello World</p><p>Hello World!</p>");
}

#[component]
fn Optional(
    #[builder(default, setter(strip_option))] borrowed: Option<&str>,
    #[builder(default, setter(strip_option))] owned: Option<String>,
) -> HtmlFragment {
    html! { <p>{borrowed}{owned}</p> }
}

#[test]
fn test_strip_option_setters() {
    let name = "World";
    let result = html! {
        <Optional />
        <Optional borrowed="Hello {name}" owned="Hello" />
    };

    assert_eq!(result.to_string(), "<p></p><p>Hello WorldHello</p>");
}

#[component]
fn Row<'a>(first: &'a str, items: &'a [u32], r#type: &'a str, last: String) -> HtmlFragment {
    html! { <p class={r#type}>{first}" "{items.len()}" "{last}</p> }
}

// The `String` behind a `&str` prop has to live until the component has run, like
// the `Vec` that the user borrows in the same tag. Here they share a lifetime.
#[test]
fn test_several_string_props_next_to_a_borrowed_temporary() {
    let name = "World";
    let result = html! {
        <Row first="Hello {name}" items={&(1..=3).collect::<Vec<u32>>()} type="row-{name}" last="!" />
    };

    assert_eq!(
        result.to_string(),
        "<p class=\"row-World\">Hello World 3 !</p>"
    );
}

#[test]
fn test_string_props_with_for_and_if() {
    let names = ["a", "b"];
    let show = true;
    let result = html! {
        <Borrowed :for={name in names} :if={show} text="Hello {name}" />
        <Owned :for={_ in names} text="Hi" />
    };

    assert_eq!(
        result.to_string(),
        "<p>Hello a</p><p>Hello b</p><p>Hi</p><p>Hi</p>"
    );
}

// `?` in a prop still returns from the function that `html!` is written in
fn parsed(input: &str) -> Result<HtmlFragment, std::num::ParseIntError> {
    Ok(html! {
        <Row first="n = {input}" items={&[input.parse::<u32>()?]} type="row" last="{input}!" />
    })
}

#[test]
fn test_question_mark_in_a_prop_next_to_string_props() {
    assert_eq!(
        parsed("7").unwrap().to_string(),
        "<p class=\"row\">n = 7 1 7!</p>"
    );
    assert!(parsed("seven").is_err());
}

// The names of the builder and the value in the generated code are the macro's own,
// so they don't get in the way of each other when a tag comes out of `macro_rules!`
macro_rules! greeting {
    ($component:ident) => {{
        let name = "World";
        html! { <$component text="Hello {name}" /> }
    }};
}

#[test]
fn test_string_props_from_macro_rules() {
    assert_eq!(greeting!(Borrowed).to_string(), "<p>Hello World</p>");
    assert_eq!(greeting!(Owned).to_string(), "<p>Hello World</p>");
}

#[component]
fn Pair<'a>(first: &'a str, second: &'a str) -> HtmlFragment {
    html! { <p>{first}" "{second}</p> }
}

// A string written out is `'static`, but it doesn't make the prop's lifetime
// `'static`: the other prop with that lifetime still takes a local
#[test]
fn test_plain_string_shares_a_lifetime_with_a_local() {
    let local = String::from("World");
    let result = html! { <Pair first="Hello" second={&local} /> };

    assert_eq!(result.to_string(), "<p>Hello World</p>");
}
