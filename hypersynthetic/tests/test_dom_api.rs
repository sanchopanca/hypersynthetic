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
    assert_eq!(Node::Element(input.clone()).to_string(), "<input disabled>");

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

// Attribute names are matched ignoring ASCII case, as in an HTML document, where
// the parser lowercases them: `ID`, `Id` and `id` are the same attribute

#[test]
fn test_attribute_names_ignore_case_when_reading() {
    let page = html! { <input ID="name" Disabled /> };
    let input = element(&page);

    assert_eq!(input.get_attribute("id"), Some("name"));
    assert_eq!(input.get_attribute("iD"), Some("name"));
    assert_eq!(input.id(), Some("name"));
    assert!(input.has_attribute("disabled"));
    assert!(input.has_attribute("DISABLED"));
    assert!(!input.has_attribute("name"));
}

#[test]
fn test_attribute_names_are_kept_as_written() {
    let page = html! { <svg viewBox="0 0 10 10" ID="logo"></svg> };
    let svg = element(&page);

    assert_eq!(svg.get_attribute("viewBox"), Some("0 0 10 10"));
    let names: Vec<&str> = svg.get_attribute_names().collect();
    assert_eq!(names, ["viewBox", "ID"]);
    assert_eq!(
        page.to_string(),
        r#"<svg viewBox="0 0 10 10" ID="logo"></svg>"#
    );
}

#[test]
fn test_set_attribute_ignores_case_and_keeps_the_spelling() {
    let mut page = html! { <svg viewBox="0 0 10 10" width="10"></svg> };

    // The attribute that's there is changed: its name stays as it was written
    element_mut(&mut page).set_attribute("viewbox", "0 0 20 20");
    assert_eq!(
        page.to_string(),
        r#"<svg viewBox="0 0 20 20" width="10"></svg>"#
    );

    // A new attribute gets the name as given
    element_mut(&mut page).set_attribute("preserveAspectRatio", "none");
    assert_eq!(
        page.to_string(),
        r#"<svg viewBox="0 0 20 20" width="10" preserveAspectRatio="none"></svg>"#
    );
}

#[test]
fn test_set_attribute_removes_duplicates_in_any_case() {
    // Names that `html!` can't see are the same: they're only known at runtime
    let (lower, mixed) = ("href", "Href");
    let mut page = html! { <a HREF="/1" title="t" {lower}="/2" {mixed}="/3"></a> };

    element_mut(&mut page).set_attribute("href", "/4");
    assert_eq!(page.to_string(), r#"<a HREF="/4" title="t"></a>"#);
}

#[test]
fn test_remove_attribute_ignores_case() {
    let lower = "href";
    let mut page = html! { <a HREF="/1" title="t" {lower}="/2"></a> };

    element_mut(&mut page).remove_attribute("Href");
    assert_eq!(page.to_string(), r#"<a title="t"></a>"#);
}

#[test]
fn test_toggle_attribute_ignores_case() {
    let mut page = html! { <input Disabled /> };
    let input = element_mut(&mut page);

    // It's there, so it's removed, not added a second time
    assert!(!input.toggle_attribute("disabled", None));
    assert_eq!(page.to_string(), "<input />");

    let input = element_mut(&mut page);
    assert!(input.toggle_attribute("READONLY", None));
    assert!(input.toggle_attribute("readonly", Some(true)));
    assert_eq!(page.to_string(), "<input READONLY />");
}

#[test]
fn test_attribute_names_only_ignore_ascii_case() {
    let mut div = ElementData::new("div");
    div.set_attribute("data-é", "lower");

    // Like the DOM, which only lowercases A to Z
    assert_eq!(div.get_attribute("data-É"), None);
    assert_eq!(div.get_attribute("DATA-é"), Some("lower"));
}

#[test]
fn test_get_element_by_id_ignores_the_case_of_the_attribute_name() {
    let mut page = html! { <div ID="x"></div><p Id="y"></p> };

    assert_eq!(page.get_element_by_id("x").unwrap().tag_name, "div");
    assert_eq!(page.get_element_by_id_mut("y").unwrap().tag_name, "p");
    // The id itself is compared exactly
    assert!(page.get_element_by_id("X").is_none());
}

#[test]
fn test_class_list_ignores_the_case_of_the_attribute_name() {
    let mut page = html! { <div Class="a"></div> };

    assert!(element(&page).class_list().contains("a"));
    assert_eq!(page.get_elements_by_class_name("a").count(), 1);
    // Class names are compared exactly
    assert_eq!(page.get_elements_by_class_name("A").count(), 0);

    // Added to the attribute that's there, not to a second `class`
    element_mut(&mut page).class_list_mut().add("b");
    assert_eq!(page.to_string(), r#"<div Class="a b"></div>"#);

    element_mut(&mut page)
        .class_list_mut()
        .remove("a")
        .remove("b");
    assert_eq!(page.to_string(), r#"<div Class=""></div>"#);
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

// In the DOM an element with an empty `id` has no id, so `getElementById("")` finds
// nothing. Otherwise an id and a lookup that both turned out empty would meet.
#[test]
fn test_get_element_by_id_never_finds_an_empty_id() {
    let slug = "";
    let mut page = html! {
        <section>
            <div id></div>
            <p id=""></p>
            <span id={slug}></span>
            <b id="x"></b>
        </section>
    };

    assert!(page.get_element_by_id("").is_none());
    assert!(page.get_element_by_id_mut("").is_none());

    // The same below an element
    let section = element_mut(&mut page);
    assert!(section.get_element_by_id("").is_none());
    assert!(section.get_element_by_id_mut("").is_none());

    // Other ids are found as before, and the empty attribute can still be read
    assert_eq!(page.get_element_by_id("x").unwrap().tag_name, "b");
    assert_eq!(
        ids(page.descendants()),
        ["-", "", "", "", "x"],
        "`id()` reports the attribute"
    );
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

// Creating text: `Node::text` escapes (the DOM's createTextNode), `Node::raw_html`
// is the explicit way to insert markup as is.

#[test]
fn test_node_text_escapes() {
    let user_input = String::from("<script>alert(1)</script> & \"quotes\"");

    assert_eq!(
        Node::text(user_input).to_string(),
        "&lt;script&gt;alert(1)&lt;/script&gt; &amp; \"quotes\""
    );
    assert_eq!(Node::text("plain").to_string(), "plain");
}

#[test]
fn test_node_text_is_what_html_creates() {
    let value = "a < b";
    let from_macro = html! { {value} };

    assert_eq!(from_macro.child_nodes(), [Node::text(value)]);
}

#[test]
fn test_node_raw_html() {
    let mut p = ElementData::new("p");
    p.add_child(Node::raw_html("<b>bold</b> &amp;"));

    assert_eq!(Node::Element(p).to_string(), "<p><b>bold</b> &amp;</p>");
}

#[test]
fn test_set_text_content() {
    let mut page = html! { <p>"old " <b>"text"</b></p> };
    let p = page.iter_elements_mut().next().unwrap();

    p.set_text_content("new <text>");

    assert_eq!(p.child_nodes(), [Node::text("new <text>")]);
    assert_eq!(page.to_string(), "<p>new &lt;text&gt;</p>");
}
