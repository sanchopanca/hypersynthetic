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
