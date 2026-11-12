// HTML inside `{...}` or `{{...}}` is inserted as nodes: an HtmlFragment, a Node or
// an ElementData, however it's wrapped. Everything else is formatted with Display,
// escaped in `{...}` and raw in `{{...}}`.
use hypersynthetic::prelude::*;
use hypersynthetic::{ElementData, Node};
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

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

// The types the DOM-style API hands out are HTML too

fn page() -> HtmlFragment {
    html! { <div><p id="a">"one"</p><p id="b">"two"</p></div> }
}

#[test]
fn test_nodes_are_inserted() {
    let page = html! { <b>"x"</b>"text" };
    let result = html! {
        <li :for={node in page.iter()}>{node}</li>
    };

    assert_eq!(result.to_string(), "<li><b>x</b></li><li>text</li>");
}

#[test]
fn test_node_by_value_is_inserted() {
    let node = Node::Element(ElementData::new("hr"));
    let result = html! { <div>{node}</div> };

    assert_eq!(result.to_string(), "<div><hr></div>");
}

#[test]
fn test_elements_are_inserted() {
    let page = page();
    let by_reference = page.get_element_by_id("b").unwrap();
    let by_value = by_reference.clone();
    let result = html! {
        <i>{by_reference}</i>
        <i>{by_value}</i>
    };

    assert_eq!(
        result.to_string(),
        r#"<i><p id="b">two</p></i><i><p id="b">two</p></i>"#
    );
}

#[test]
fn test_optional_elements_are_inserted() {
    let page = page();
    let result = html! {
        <i>{page.get_element_by_id("b")}</i>
        <i>{page.get_element_by_id("missing")}</i>
        <i>{&page.get_element_by_id("a")}</i>
    };

    assert_eq!(
        result.to_string(),
        r#"<i><p id="b">two</p></i><i></i><i><p id="a">one</p></i>"#
    );
}

#[test]
fn test_inserted_elements_keep_their_structure() {
    let page = page();
    let result = html! { <section>{page.get_element_by_id("b")}</section> };

    let found = result.get_element_by_id("b").unwrap();
    assert_eq!(found.tag_name, "p");
}

// However the HTML is wrapped

#[test]
fn test_references_to_references() {
    let (a, b) = (html! { <i>"a"</i> }, html! { <i>"b"</i> });
    let frags: Vec<&HtmlFragment> = vec![&a, &b];
    let result = html! {
        <li :for={frag in frags.iter()}>{frag}</li>
    };

    assert_eq!(result.to_string(), "<li><i>a</i></li><li><i>b</i></li>");
}

#[test]
fn test_mutable_reference() {
    let mut frag = html! { <b>"x"</b> };
    let frag = &mut frag;
    let result = html! { <p>{frag}</p> };

    assert_eq!(result.to_string(), "<p><b>x</b></p>");
}

#[test]
fn test_smart_pointers() {
    let frag = html! { <b>"x"</b> };
    let boxed = Box::new(frag.clone());
    let rc = Rc::new(frag.clone());
    let arc = Arc::new(frag.clone());
    let cow: Cow<HtmlFragment> = Cow::Borrowed(&frag);
    let result = html! { <p>{boxed}{rc}{arc}{cow}</p> };

    assert_eq!(
        result.to_string(),
        "<p><b>x</b><b>x</b><b>x</b><b>x</b></p>"
    );
}

#[test]
fn test_wrappers_inside_wrappers() {
    let frag = html! { <b>"x"</b> };
    let optional_reference = Some(&frag);
    let shared: Option<Arc<HtmlFragment>> = Some(Arc::new(frag.clone()));
    let nothing: Option<Box<Node>> = None;
    let result = html! { <p>{&optional_reference}{shared}{nothing}</p> };

    assert_eq!(result.to_string(), "<p><b>x</b><b>x</b></p>");
}

// To show the markup as text, make it a string first

#[test]
fn test_markup_as_text() {
    let page = page();
    let element = page.get_element_by_id("b").unwrap();
    let result = html! { <code>{element.to_string()}</code> };

    assert_eq!(
        result.to_string(),
        r#"<code>&lt;p id="b"&gt;two&lt;/p&gt;</code>"#
    );
}

// An attribute's value is text, whatever the type

#[test]
fn test_element_as_attribute_value_is_text() {
    let page = page();
    let element = page.get_element_by_id("b").unwrap();
    let result = html! { <i title={element}></i> };

    assert_eq!(
        result.to_string(),
        r#"<i title="&lt;p id=&quot;b&quot;&gt;two&lt;/p&gt;"></i>"#
    );
}
