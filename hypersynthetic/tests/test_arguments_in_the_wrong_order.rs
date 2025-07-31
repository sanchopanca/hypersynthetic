use hypersynthetic::prelude::*;

#[component]
fn Component(val1: &str, val2: i32) -> HtmlFragment {
    html! {
        <div>
            <p>{val1}</p>
            <p>{val2 + 1}</p>
        </div>
    }
}

#[component]
fn ComponentWithArgsOfTheSameType(val1: &str, val2: &str) -> HtmlFragment {
    html! {
        <div>
            <p>{"Val1: "}{val1}</p>
            <p>{"Val2: "}{val2}</p>
        </div>
    }
}

#[test]
fn test_arguments_in_the_wrong_order() {
    let result1 = html! {
        <Component val1="test" val2={-1} />
    };

    let result2 = html! {
        <Component val2={-1} val1="test" />
    };

    let string_representation1 = result1.to_string();
    let string_representation2 = result2.to_string();

    assert_eq!(string_representation1, string_representation2);
}

#[test]
fn test_arguments_in_the_wrong_order_of_the_same_type() {
    let result1 = html! {
        <ComponentWithArgsOfTheSameType val1="one" val2="two" />
    };

    let result2 = html! {
        <ComponentWithArgsOfTheSameType val2="two" val1="one" />
    };

    let string_representation1 = result1.to_string();
    let string_representation2 = result2.to_string();

    assert_eq!(string_representation1, string_representation2);
}
