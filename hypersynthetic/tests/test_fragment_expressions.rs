// An HtmlFragment inside `{...}` or `{{...}}` is inserted as nodes. Everything
// else is formatted with Display, escaped in `{...}` and raw in `{{...}}`.
use hypersynthetic::prelude::*;

#[test]
fn test_fragment_in_single_braces_is_not_escaped() {
    let frag = html! { <b>"bold"</b> };
    let result = html! {
        <p>{frag}</p>
    };

    assert_eq!(result.to_string(), "<p><b>bold</b></p>");
}

#[test]
fn test_fragment_references() {
    let frags = vec![html! { <i>"a"</i> }, html! { <i>"b"</i> }];
    let result = html! {
        <li :for={frag in &frags}>{frag}</li>
        <p>{&frags[0]}</p>
    };

    assert_eq!(
        result.to_string(),
        "<li><i>a</i></li><li><i>b</i></li><p><i>a</i></p>"
    );
}

#[test]
fn test_fragments_keep_their_structure() {
    let frag = html! { <b>"bold"</b> };
    for result in [html! { <div>{frag}</div> }, html! { <div>{{ frag }}</div> }] {
        let div = result.iter_elements().next().unwrap();
        let child = div.children.iter_elements().next().unwrap();
        assert_eq!(child.tag_name, "b");
    }
}

#[test]
fn test_expressions_are_borrowed_not_moved() {
    let frag = html! { <b>"bold"</b> };
    let text = String::from("<i>");
    let result = html! {
        <p>{frag}{text}</p>
        <p>{{ frag }}{{ text }}</p>
    };

    assert_eq!(
        result.to_string(),
        "<p><b>bold</b>&lt;i&gt;</p><p><b>bold</b><i></p>"
    );
    // Both are still usable here
    assert_eq!(frag.to_string(), "<b>bold</b>");
    assert_eq!(text, "<i>");
}
