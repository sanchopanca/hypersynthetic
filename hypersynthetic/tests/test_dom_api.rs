//! DOM-like methods on elements, named like the DOM's in snake_case.

use hypersynthetic::prelude::*;
use hypersynthetic::{ElementData, Node};

fn element(fragment: &HtmlFragment) -> &ElementData {
    fragment.iter_elements().next().unwrap()
}

fn element_mut(fragment: &mut HtmlFragment) -> &mut ElementData {
    fragment.iter_elements_mut().next().unwrap()
}

#[test]
fn test_toggle_attribute() {
    let mut input = ElementData::new("input");

    assert!(input.toggle_attribute("disabled", None));
    assert_eq!(
        Node::Element(input.clone()).to_string(),
        "<input disabled></input>"
    );

    assert!(!input.toggle_attribute("disabled", None));
    assert!(!input.has_attribute("disabled"));
}

#[test]
fn test_toggle_attribute_with_force() {
    let mut input = ElementData::new("input");

    assert!(!input.toggle_attribute("required", Some(false)));
    assert!(!input.has_attribute("required"));

    assert!(input.toggle_attribute("required", Some(true)));
    assert!(input.toggle_attribute("required", Some(true)));
    assert!(input.has_attribute("required"));

    assert!(!input.toggle_attribute("required", Some(false)));
    assert!(!input.has_attribute("required"));
}

#[test]
fn test_id() {
    let page = html! { <main id="content"></main><nav></nav> };
    let mut elements = page.iter_elements();

    assert_eq!(elements.next().unwrap().id(), Some("content"));
    assert_eq!(elements.next().unwrap().id(), None);
}

#[test]
fn test_get_attribute_names() {
    let page = html! { <a href="/" class="link" download></a> };

    let names: Vec<&str> = element(&page).get_attribute_names().collect();
    assert_eq!(names, ["href", "class", "download"]);
}

#[test]
fn test_class_list() {
    let page = html! { <p class="  a\tb  a c "></p><p></p> };
    let mut elements = page.iter_elements();

    let classes = elements.next().unwrap().class_list();
    assert!(classes.contains("b"));
    assert!(!classes.contains("d"));
    assert_eq!(classes.iter().collect::<Vec<_>>(), ["a", "b", "c"]);
    assert_eq!(classes.len(), 3);

    let none = elements.next().unwrap().class_list();
    assert!(none.is_empty());
    assert!(!none.contains("a"));
}

#[test]
fn test_class_list_add_and_remove() {
    let mut page = html! { <p class="a  b" id="x"></p> };
    let p = element_mut(&mut page);

    p.class_list_mut().add("c").add("a");
    assert_eq!(p.get_attribute("class"), Some("a b c"));

    p.class_list_mut().remove("b");
    assert_eq!(page.to_string(), r#"<p class="a c" id="x"></p>"#);
}

#[test]
fn test_class_list_add_creates_the_attribute() {
    let mut p = ElementData::new("p");
    p.class_list_mut().add("new");
    assert_eq!(Node::Element(p).to_string(), r#"<p class="new"></p>"#);
}

#[test]
fn test_class_list_remove_like_the_dom() {
    // Removing the last class leaves an empty attribute, as in browsers
    let mut page = html! { <p class="only"></p> };
    element_mut(&mut page).class_list_mut().remove("only");
    assert_eq!(page.to_string(), r#"<p class=""></p>"#);

    // Removing from an element without a class attribute doesn't create one
    let mut p = ElementData::new("p");
    p.class_list_mut().remove("missing");
    assert!(!p.has_attribute("class"));
}

#[test]
fn test_class_list_toggle() {
    let mut p = ElementData::new("p");
    let mut classes = p.class_list_mut();

    assert!(classes.toggle("open", None));
    assert!(classes.contains("open"));
    assert!(!classes.toggle("open", None));
    assert!(!classes.contains("open"));
    assert!(classes.toggle("open", Some(true)));
    assert!(classes.toggle("open", Some(true)));
    assert!(!classes.toggle("open", Some(false)));
    assert!(!classes.toggle("open", Some(false)));
}

#[test]
fn test_class_list_replace() {
    let mut page = html! { <p class="a b c b"></p> };
    let p = element_mut(&mut page);

    assert!(p.class_list_mut().replace("b", "x"));
    assert_eq!(p.get_attribute("class"), Some("a x c"));

    assert!(!p.class_list_mut().replace("missing", "y"));
    assert_eq!(p.get_attribute("class"), Some("a x c"));

    // The new class already there: it stays where it was first
    assert!(p.class_list_mut().replace("c", "a"));
    assert_eq!(p.get_attribute("class"), Some("a x"));
}

#[test]
#[should_panic(expected = "class names can't be empty")]
fn test_class_list_rejects_empty_names() {
    ElementData::new("p").class_list_mut().add("");
}

#[test]
#[should_panic(expected = "contains whitespace")]
fn test_class_list_rejects_names_with_whitespace() {
    ElementData::new("p").class_list_mut().add("a b");
}

// Searching: tree order (depth-first, parents before children), like the DOM.

fn ids<'a>(elements: impl Iterator<Item = &'a ElementData>) -> Vec<&'a str> {
    elements
        .map(|element| element.id().unwrap_or("-"))
        .collect()
}

fn page() -> HtmlFragment {
    html! {
        <div id="a" class="box">
            "text"
            <p id="b" class="note box"><span id="c" class="box note big"></span></p>
            <p id="d"></p>
        </div>
        <footer id="e" class="note"></footer>
        <p id="b"></p>
    }
}

#[test]
fn test_descendants() {
    let page = page();
    assert_eq!(ids(page.descendants()), ["a", "b", "c", "d", "e", "b"]);

    // An element's descendants don't include the element itself
    let div = page.get_element_by_id("a").unwrap();
    assert_eq!(ids(div.descendants()), ["b", "c", "d"]);
}

#[test]
fn test_get_element_by_id() {
    let page = page();

    assert_eq!(page.get_element_by_id("c").unwrap().tag_name, "span");
    // The first one in tree order
    assert_eq!(
        page.get_element_by_id("b").unwrap().get_attribute("class"),
        Some("note box")
    );
    assert!(page.get_element_by_id("missing").is_none());
}

#[test]
fn test_get_elements_by_tag_name() {
    let page = page();

    assert_eq!(ids(page.get_elements_by_tag_name("p")), ["b", "d", "b"]);
    // Case-insensitive for HTML elements
    assert_eq!(ids(page.get_elements_by_tag_name("P")), ["b", "d", "b"]);
    assert_eq!(ids(page.get_elements_by_tag_name("*")).len(), 6);
    assert_eq!(ids(page.get_elements_by_tag_name("table")), [] as [&str; 0]);
}

#[test]
fn test_get_elements_by_class_name() {
    let page = page();

    assert_eq!(ids(page.get_elements_by_class_name("box")), ["a", "b", "c"]);
    // All of the classes, in any order
    assert_eq!(
        ids(page.get_elements_by_class_name(" note  box ")),
        ["b", "c"]
    );
    assert_eq!(ids(page.get_elements_by_class_name("")), [] as [&str; 0]);
}

#[test]
fn test_get_element_by_id_mut() {
    let original = page();
    let mut page = original.clone();

    page.get_element_by_id_mut("c")
        .unwrap()
        .class_list_mut()
        .add("changed");

    assert!(
        page.get_element_by_id("c")
            .unwrap()
            .class_list()
            .contains("changed")
    );
    // Copy-on-write: the clone it came from is unchanged
    assert!(
        !original
            .get_element_by_id("c")
            .unwrap()
            .class_list()
            .contains("changed")
    );
    assert!(page.get_element_by_id_mut("missing").is_none());
}

#[test]
fn test_for_each_descendant_mut() {
    let mut page = page();
    let mut visited = Vec::new();

    page.for_each_descendant_mut(|element| {
        visited.push(element.id().unwrap_or("-").to_owned());
        if element.class_list().contains("note") {
            element.class_list_mut().add("seen");
        }
    });

    assert_eq!(visited, ["a", "b", "c", "d", "e", "b"]);
    assert_eq!(
        ids(page.get_elements_by_class_name("seen")),
        ["b", "c", "e"]
    );
}

#[test]
fn test_for_each_descendant_mut_sees_added_children() {
    // The callback runs before an element's children are visited, so it can add some
    let mut page = html! { <ul></ul> };
    let mut tags = Vec::new();

    page.for_each_descendant_mut(|element| {
        tags.push(element.tag_name.to_string());
        if element.tag_name == "ul" {
            element.add_child(Node::Element(ElementData::new("li")));
        }
    });

    assert_eq!(tags, ["ul", "li"]);
}
