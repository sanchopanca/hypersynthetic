use hypersynthetic::prelude::*;

#[component]
fn OrangeDiv(inner_block: HtmlFragment) -> HtmlFragment {
    html! {
        <div class="orange round">
            {{ inner_block }}
        </div>
    }
}

#[test]
fn test_slots() {
    let data = "Hello, world!";
    let result = html! {
        <OrangeDiv>
            <p>{ data }</p>
        </OrangeDiv>
    };

    assert_eq!(
        result.to_string(),
        "<div class=\"orange round\"><p>Hello, world!</p></div>"
    );
}

#[test]
fn test_slots_deep() {
    let result = html! {
        <div :for={i in 0..=1}>
            <OrangeDiv>
                <p>{ format!("hello {}", i) }</p>
            </OrangeDiv>
        </div>
    };

    assert_eq!(
        result.to_string(),
        "<div><div class=\"orange round\"><p>hello 0</p></div></div><div><div class=\"orange round\"><p>hello 1</p></div></div>"
    );
}

#[test]
fn test_slots_with_for() {
    let result = html! {
        <OrangeDiv :for={i in 0..=1}>
            <p>{ i }</p>
        </OrangeDiv>
    };

    assert_eq!(
        result.to_string(),
        "<div class=\"orange round\"><p>0</p></div><div class=\"orange round\"><p>1</p></div>"
    );
}

#[component]
fn ColorfulDiv(inner_block: HtmlFragment, color: &str) -> HtmlFragment {
    html! {
        <div class="{color} round">
            {{ inner_block }}
        </div>
    }
}

#[test]
fn test_slots_with_props() {
    let result = html! {
        <ColorfulDiv color="blue">
            <p>{ "Hello, world!" }</p>
        </ColorfulDiv>
    };

    assert_eq!(
        result.to_string(),
        "<div class=\"blue round\"><p>Hello, world!</p></div>"
    );
}
