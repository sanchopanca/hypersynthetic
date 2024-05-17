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

mod inner {
    use hypersynthetic::prelude::*;
    #[component]
    pub fn InnerComponent(val: &str) -> HtmlFragment {
        html! {
            <span>{val}</span>
        }
    }

    #[component]
    pub fn Div(content: HtmlFragment) -> HtmlFragment {
        html! {
            <div>{{ content }}</div>
        }
    }
}

#[test]
fn test_component() {
    let result = html! {
        <Component val1="Hello" val2={41} />
    };

    let string_representation = result.to_string();

    let expected = "<div><p>Hello</p><p>42</p></div>";

    assert_eq!(string_representation, expected);
}

#[test]
fn test_component_as_a_child() {
    let result = html! {
        <div>
            <Component val1="test" val2={-1} />
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><div><p>test</p><p>0</p></div></div>";

    assert_eq!(string_representation, expected);
}

#[test]
fn test_inner_component() {
    let result = html! {
        <inner::InnerComponent val="test" />
    };

    let string_representation = result.to_string();

    let expected = "<span>test</span>";

    assert_eq!(string_representation, expected);
}

#[test]
fn test_inner_component_with_slot_argument() {
    let result = html! {
        <inner::Div>
            <inner::InnerComponent val="test" />
        </inner::Div>
    };

    let string_representation = result.to_string();

    let expected = "<div><span>test</span></div>";

    assert_eq!(string_representation, expected);
}
