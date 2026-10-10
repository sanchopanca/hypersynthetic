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
