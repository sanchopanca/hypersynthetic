use hypersynthetic::prelude::*;

#[test]
fn test_set_attribute() {
    let mut div = html! {
        <div>
            <p></p>
        </div>
    };

    let div_element = div.iter_elements_mut().next().unwrap();

    div_element.set_attribute("class".to_string(), "container".to_string());

    assert_eq!(div.to_string(), r#"<div class="container"><p></p></div>"#);
}

#[test]
fn test_has_attribute() {
    let div = html! {
        <div class="container">
            <p></p>
        </div>
    };

    let div_element = div.iter_elements().next().unwrap();

    assert!(div_element.has_attribute("class"));
}

#[test]
fn test_get_attribute() {
    let div = html! {
        <div itemscope class="container">
            <p></p>
        </div>
    };

    let div_element = div.iter_elements().next().unwrap();

    assert_eq!(
        div_element.get_attribute("class"),
        Some("container".to_string())
    );

    assert_eq!(div_element.get_attribute("id"), None);

    assert_eq!(div_element.get_attribute("itemscope"), Some("".to_string()));
}

#[test]
fn test_iter() {
    let div = html! {
        <div>
            <p></p>
        </div>
        {"Hello there!"}
        <div></div>
    };

    let nodes = div.iter().count();

    assert_eq!(nodes, 3);
}

#[test]
fn test_iter_mut() {
    let mut div = html! {
        <div>
            <p></p>
        </div>
        <div></div>
    };

    for node in div.iter_mut() {
        if let hypersynthetic::Node::Element(element) = node {
            element.set_attribute("class".to_string(), "container".to_string());
        }
    }

    assert_eq!(
        div.to_string(),
        r#"<div class="container"><p></p></div><div class="container"></div>"#
    );
}

#[test]
fn test_iter_elements() {
    let div = html! {
        <div>
            <p></p>
        </div>
        {"Hello there!"}
        <div></div>
    };

    let elements = div.iter_elements().count();

    assert_eq!(elements, 2);
}

#[test]
fn test_iter_elements_mut() {
    let mut div = html! {
        <div>
            <p></p>
        </div>
        <div></div>
    };

    for element in div.iter_elements_mut() {
        element.set_attribute("class".to_string(), "container".to_string());
    }

    assert_eq!(
        div.to_string(),
        r#"<div class="container"><p></p></div><div class="container"></div>"#
    );
}

#[test]
fn test_remove_attribute() {
    let mut div = html! {
        <div class="container">
            <p></p>
        </div>
    };

    let div_element = div.iter_elements_mut().next().unwrap();
    div_element.remove_attribute("class");

    assert_eq!(div.to_string(), r#"<div><p></p></div>"#);
}

#[test]
fn test_set_attribute_replaces_the_value() {
    let mut link = html! { <a href="/1"></a> };
    let element = link.iter_elements_mut().next().unwrap();

    element.set_attribute("href".to_owned(), "/2".to_owned());
    element.set_attribute("href".to_owned(), "/3".to_owned());

    assert_eq!(link.to_string(), r#"<a href="/3"></a>"#);
}

#[test]
fn test_set_attribute_keeps_the_position() {
    let mut input = html! { <input class="x" disabled id="y" /> };
    let element = input.iter_elements_mut().next().unwrap();

    element.set_attribute("disabled".to_owned(), "disabled".to_owned());

    assert_eq!(
        input.to_string(),
        r#"<input class="x" disabled="disabled" id="y" />"#
    );
}

#[test]
fn test_set_attribute_removes_duplicates() {
    let mut link = html! { <a href="/1" title="t" href="/2"></a> };
    let element = link.iter_elements_mut().next().unwrap();

    element.set_attribute("href".to_owned(), "/3".to_owned());

    assert_eq!(link.to_string(), r#"<a href="/3" title="t"></a>"#);
}
