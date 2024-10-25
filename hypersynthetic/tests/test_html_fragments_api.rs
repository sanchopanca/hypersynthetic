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
