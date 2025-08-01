use hypersynthetic::prelude::*;

#[component]
fn Component(values: Vec<&str>) -> HtmlFragment {
    html! {
        <div>
            <p :for={value in values}>{value}</p>
        </div>
    }
}

#[test]
fn test_arguments_with_refs() {
    let strings = vec!["one", "two"];
    let result = html! {
        <Component values={strings} />
    };

    let string_representation = result.to_string();

    let expected = "<div><p>one</p><p>two</p></div>";
    assert_eq!(string_representation, expected);
}

#[component]
fn BiggerComponent(xs: &[&str], ys: &[&str]) -> HtmlFragment {
    html! {
        <div>
            <p :for={x in xs}>{x}</p>
            <p :for={y in ys}>{y}</p>
        </div>
    }
}

#[test]
fn test_two_arguments_with_refs() {
    let strings = vec!["one", "two"];
    let result = html! {
        <BiggerComponent xs={&strings} ys={&strings} />
    };

    let string_representation = result.to_string();

    let expected = "<div><p>one</p><p>two</p><p>one</p><p>two</p></div>";
    assert_eq!(string_representation, expected);
}
