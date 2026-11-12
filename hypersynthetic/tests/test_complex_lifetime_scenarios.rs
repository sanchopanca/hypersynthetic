use hypersynthetic::prelude::*;
use std::fmt::Display;

#[component]
fn TraitObjectComponent(formatter: &dyn Display) -> HtmlFragment {
    html! {
        <div>{formatter}</div>
    }
}

#[test]
fn test_trait_object() {
    let number = 42;
    let result = html! {
        <TraitObjectComponent formatter={&number} />
    };
    assert_eq!(result.to_string(), "<div>42</div>");
}

#[component]
fn OptionComponent(maybe_text: Option<&str>) -> HtmlFragment {
    html! {
        <div>
            {maybe_text.unwrap_or("Nothing")}
        </div>
    }
}

#[test]
fn test_option_with_ref() {
    let result1 = html! {
        <OptionComponent maybe_text={Some("Hello")} />
    };
    assert_eq!(result1.to_string(), "<div>Hello</div>");

    let result2 = html! {
        <OptionComponent maybe_text={None} />
    };
    assert_eq!(result2.to_string(), "<div>Nothing</div>");
}

#[component]
fn ResultComponent(result: Result<&str, &str>) -> HtmlFragment {
    html! {
        <div>
            {match result {
                Ok(text) => text,
                Err(err) => err
            }}
        </div>
    }
}

#[test]
fn test_result_with_refs() {
    let result1 = html! {
        <ResultComponent result={Ok("Success")} />
    };
    assert_eq!(result1.to_string(), "<div>Success</div>");

    let result2 = html! {
        <ResultComponent result={Err("Error")} />
    };
    assert_eq!(result2.to_string(), "<div>Error</div>");
}

#[component]
fn FnPtrComponent(transform: fn(&str) -> String) -> HtmlFragment {
    html! {
        <div>{transform("hello")}</div>
    }
}

#[test]
fn test_fn_ptr() {
    fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    let result = html! {
        <FnPtrComponent transform={uppercase} />
    };
    assert_eq!(result.to_string(), "<div>HELLO</div>");
}

#[component]
fn ComplexComponent(first: &str, second: &str, items: Vec<&str>) -> HtmlFragment {
    html! {
        <div>
            <p>{first}</p>
            <p>{second}</p>
            <ul>
                <li :for={item in items}>{item}</li>
            </ul>
        </div>
    }
}

#[test]
fn test_complex_lifetimes() {
    let items = vec!["one", "two", "three"];
    let result = html! {
        <ComplexComponent first="First" second="Second" items={items} />
    };

    let expected =
        "<div><p>First</p><p>Second</p><ul><li>one</li><li>two</li><li>three</li></ul></div>";
    assert_eq!(result.to_string(), expected);
}

#[component]
fn MutRefComponent(value: &mut String) -> HtmlFragment {
    value.push_str(" - modified");
    html! {
        <div>{&*value}</div>
    }
}

#[test]
fn test_mut_ref() {
    let mut text = String::from("Original");
    let result = html! {
        <MutRefComponent value={&mut text} />
    };
    assert_eq!(result.to_string(), "<div>Original - modified</div>");
    assert_eq!(text, "Original - modified");
}

#[component]
fn NestedComponent(data: Option<Result<&str, &str>>) -> HtmlFragment {
    html! {
        <div>
            {match data {
                Some(Ok(text)) => text,
                Some(Err(err)) => err,
                None => "No data"
            }}
        </div>
    }
}

#[test]
fn test_nested_option_result() {
    let result1 = html! {
        <NestedComponent data={Some(Ok("Success"))} />
    };
    assert_eq!(result1.to_string(), "<div>Success</div>");

    let result2 = html! {
        <NestedComponent data={Some(Err("Error"))} />
    };
    assert_eq!(result2.to_string(), "<div>Error</div>");

    let result3 = html! {
        <NestedComponent data={None} />
    };
    assert_eq!(result3.to_string(), "<div>No data</div>");
}

#[component]
fn ExplicitlyElided(text: &'_ str, cow: std::borrow::Cow<'_, str>) -> HtmlFragment {
    html! {
        <p>{text}{cow}</p>
    }
}

#[test]
fn test_explicitly_elided_lifetimes() {
    let result = html! {
        <ExplicitlyElided text="a" cow={std::borrow::Cow::Borrowed("b")} />
    };
    assert_eq!(result.to_string(), "<p>ab</p>");
}

#[component]
fn ElidedTraitObjectBound(value: Box<dyn Display + '_>) -> HtmlFragment {
    html! {
        <p>{value}</p>
    }
}

#[test]
fn test_elided_trait_object_bound() {
    let number = 42;
    let result = html! {
        <ElidedTraitObjectBound value={Box::new(&number)} />
    };
    assert_eq!(result.to_string(), "<p>42</p>");
}

#[component]
fn NamedAndElided<'b>(named: &'b str, elided: &str) -> HtmlFragment {
    html! {
        <p>{named}{elided}</p>
    }
}

#[component]
fn NamedAAndElided<'a>(named: &'a str, elided: &str) -> HtmlFragment {
    html! {
        <p>{named}{elided}</p>
    }
}

#[test]
fn test_declared_lifetime_and_elided_reference() {
    let result = html! {
        <NamedAndElided named="a" elided="b" />
        <NamedAAndElided named="c" elided="d" />
    };
    assert_eq!(result.to_string(), "<p>ab</p><p>cd</p>");
}

#[component]
fn GenericWithRef<T: Display>(value: &T) -> HtmlFragment {
    html! {
        <p>{value}</p>
    }
}

#[test]
fn test_generic_component_with_reference() {
    let result = html! {
        <GenericWithRef value={&42} />
    };
    assert_eq!(result.to_string(), "<p>42</p>");
}

#[component]
fn BoxedClosure(transform: Box<dyn Fn(&str) -> String>) -> HtmlFragment {
    html! {
        <p>{transform("hello")}</p>
    }
}

#[test]
fn test_closure_with_reference_argument() {
    let result = html! {
        <BoxedClosure transform={Box::new(|s: &str| s.to_uppercase())} />
    };
    assert_eq!(result.to_string(), "<p>HELLO</p>");
}

// Each elided lifetime is its own, as in any function. If they were all the same one,
// `&mut Counter<'_>` would be `&'a mut Counter<'a>`: a borrow that lasts as long as the
// counter itself, so it could be passed to a component only once.

struct Counter<'x> {
    label: &'x str,
    count: u32,
}

#[component]
fn Count(counter: &mut Counter<'_>) -> HtmlFragment {
    counter.count += 1;
    html! { <p>{counter.label}": "{counter.count}</p> }
}

#[test]
fn test_mut_ref_to_a_type_with_a_lifetime_can_be_used_again() {
    let label = String::from("clicks");
    let mut counter = Counter {
        label: &label,
        count: 0,
    };

    let first = html! { <Count counter={&mut counter} /> };
    let second = html! { <Count counter={&mut counter} /> };

    assert_eq!(first.to_string(), "<p>clicks: 1</p>");
    assert_eq!(second.to_string(), "<p>clicks: 2</p>");
    assert_eq!(counter.count, 2);
}

#[component]
fn TakeLast(items: &mut Vec<&str>) -> HtmlFragment {
    let last = items.pop().unwrap_or("nothing");
    html! { <li>{last}</li> }
}

#[test]
fn test_mut_ref_to_a_vec_of_refs_can_be_used_again() {
    let mut items = vec!["a", "b"];

    let first = html! { <TakeLast items={&mut items} /> };
    items.push("c");
    let second = html! { <TakeLast items={&mut items} /> };

    assert_eq!(first.to_string(), "<li>b</li>");
    assert_eq!(second.to_string(), "<li>c</li>");
    assert_eq!(items, ["a"]);
}

// A name the macro gives to an elided lifetime must not be one the function uses
// itself. `for<'a>` declares `'a` wherever it's written, and a name that's already in
// scope can't be declared again (E0496).

#[component]
fn Apply(transform: Box<dyn for<'a> Fn(&'a str) -> &'a str>, text: &str) -> HtmlFragment {
    html! { <p>{transform(text)}</p> }
}

#[component]
fn ApplyGeneric<F>(transform: F, text: &str) -> HtmlFragment
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    html! { <p>{transform(text)}</p> }
}

// `'r#a` is another spelling of `'a`
#[component]
fn ApplyRaw(transform: for<'r#a> fn(&'r#a str) -> &'r#a str, text: &str) -> HtmlFragment {
    html! { <p>{transform(text)}</p> }
}

#[test]
fn test_for_lifetime_in_the_signature_and_elided_reference() {
    let result = html! {
        <Apply transform={Box::new(str::trim)} text=" a " />
        <ApplyGeneric transform={str::trim} text=" b " />
        <ApplyRaw transform={str::trim} text=" c " />
    };
    assert_eq!(result.to_string(), "<p>a</p><p>b</p><p>c</p>");
}

#[component]
fn Trimmed(text: &str) -> HtmlFragment {
    let trim: &dyn for<'a> Fn(&'a str) -> &'a str = &str::trim;
    html! { <p>{trim(text)}</p> }
}

// Inside a macro's arguments, where the function's syntax tree has only tokens
#[component]
fn TrimmedTwice(text: &str) -> HtmlFragment {
    // An array would do, but the test needs a macro
    #[allow(clippy::useless_vec)]
    let steps = vec![
        Box::new(str::trim_start) as Box<dyn for<'a> Fn(&'a str) -> &'a str>,
        Box::new(str::trim_end),
    ];
    let text = steps.iter().fold(text, |text, step| step(text));
    html! { <p>{text}</p> }
}

#[test]
fn test_for_lifetime_in_the_body_and_elided_reference() {
    let result = html! {
        <Trimmed text=" a " />
        <TrimmedTwice text=" b " />
    };
    assert_eq!(result.to_string(), "<p>a</p><p>b</p>");
}
