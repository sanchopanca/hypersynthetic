use hypersynthetic::prelude::*;

#[component]
fn OrangeDiv(children: HtmlFragment) -> HtmlFragment {
    html! {
        <div class="orange round">
            {{ children }}
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
fn ColorfulDiv(children: HtmlFragment, color: &str) -> HtmlFragment {
    html! {
        <div class="{color} round">
            {{ children }}
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

#[component]
fn IgnoresSlot(#[allow(unused_variables)] children: HtmlFragment, text: &str) -> HtmlFragment {
    html! { <p>{text}</p> }
}

#[test]
fn test_unused_children() {
    let result = html! {
        <IgnoresSlot text="kept">
            <span>"dropped"</span>
        </IgnoresSlot>
    };

    assert_eq!(result.to_string(), "<p>kept</p>");
}

#[test]
fn test_slot_component_without_children() {
    let result = html! {
        <OrangeDiv />
        <OrangeDiv></OrangeDiv>
        <ColorfulDiv color="red" />
    };

    assert_eq!(
        result.to_string(),
        "<div class=\"orange round\"></div>\
         <div class=\"orange round\"></div>\
         <div class=\"red round\"></div>"
    );
}

#[test]
fn test_slot_passed_as_children_prop() {
    let content = html! { <b>"bold"</b> };
    let result = html! {
        <OrangeDiv children={content} />
    };

    assert_eq!(
        result.to_string(),
        "<div class=\"orange round\"><b>bold</b></div>"
    );
}

// Only the parameter named `children` receives the content between the tags.
// Other `HtmlFragment` parameters are ordinary props, in any position.

#[component]
fn Layout(header: HtmlFragment, footer: HtmlFragment) -> HtmlFragment {
    html! { <header>{header}</header><footer>{footer}</footer> }
}

#[test]
fn test_fragment_props_are_ordinary_props() {
    let result = html! {
        <Layout header={html! { <h1>"Title"</h1> }} footer={html! { "(c)" }} />
    };

    assert_eq!(
        result.to_string(),
        "<header><h1>Title</h1></header><footer>(c)</footer>"
    );
}

#[component]
fn Panel(title: &str, header: HtmlFragment, children: HtmlFragment) -> HtmlFragment {
    html! { <section><h2>{title}</h2>{header}<div>{children}</div></section> }
}

#[test]
fn test_children_in_any_position() {
    let result = html! {
        <Panel title="T" header={html! { <i>"h"</i> }}>
            <p>"body"</p>
        </Panel>
        <Panel title="Empty" header={html! {}} />
    };

    assert_eq!(
        result.to_string(),
        "<section><h2>T</h2><i>h</i><div><p>body</p></div></section>\
         <section><h2>Empty</h2><div></div></section>"
    );
}
