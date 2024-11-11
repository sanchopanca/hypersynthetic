use hypersynthetic::prelude::*;

#[component]
fn Dialog(text: &str) -> HtmlFragment {
    html! {
        <div>
            <p>{text}</p>
        </div>
    }
}

#[component]
fn Dialog2(content: HtmlFragment) -> HtmlFragment {
    html! {
        <div>
            {{ content }}
        </div>
    }
}

#[test]
fn test_if() {
    let truth = true;

    let result = html! {
        <div>
            <p :if={truth}>
                "Yes"
            </p>
        </div>
    };

    let expected = "<div><p>Yes</p></div>";
    assert_eq!(result.to_string(), expected);

    let result = html! {
        <div>
            <p :if={!truth}>
                "No"
            </p>
        </div>
    };

    let expected = "<div></div>";
    assert_eq!(result.to_string(), expected);
}

#[test]
fn test_if_on_a_component() {
    let truth = true;

    let result = html! {
        <Dialog :if={truth} text="Yes" />
    };

    let expected = "<div><p>Yes</p></div>";
    assert_eq!(result.to_string(), expected);
}

#[test]
fn test_if_self_closing_tag() {
    let truth = true;

    let result = html! {
        <hr :if={truth} />
        <br :if={!truth} />
    };

    let expected = "<hr />";
    assert_eq!(result.to_string(), expected);
}

#[test]
fn test_if_on_a_component_with_slot() {
    let truth = true;

    let result = html! {
        <Dialog2 :if={truth}>
            <p>"Yes"</p>
        </Dialog2>
    };

    let expected = "<div><p>Yes</p></div>";
    assert_eq!(result.to_string(), expected);

    let result = html! {
        <Dialog2 :if={!truth}>
            <p>"No"</p>
        </Dialog2>
    };

    let expected = "";
    assert_eq!(result.to_string(), expected);
}

#[test]
fn test_if_and_for() {
    let truth = true;

    let result = html! {
        <p :if={truth} :for={i in 1..=3}>
            {i}
        </p>
    };

    let expected = "<p>1</p><p>2</p><p>3</p>";
    assert_eq!(result.to_string(), expected);

    let result = html! {
        <p :if={!truth} :for={i in 1..=3}>
            {i}
        </p>
    };

    let expected = "";
    assert_eq!(result.to_string(), expected);
}

#[test]
fn test_if_and_for_on_a_component() {
    let truth = true;

    let result = html! {
        <Dialog :if={truth} :for={i in 1..=3} text={&i.to_string()} />
    };

    let expected = "<div><p>1</p></div><div><p>2</p></div><div><p>3</p></div>";
    assert_eq!(result.to_string(), expected);

    let result = html! {
        <Dialog :if={!truth} :for={i in 1..=3} text={&i.to_string()} />
    };

    let expected = "";
    assert_eq!(result.to_string(), expected);
}
