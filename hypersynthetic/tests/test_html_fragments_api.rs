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

    assert_eq!(div_element.get_attribute("class"), Some("container"));

    assert_eq!(div_element.get_attribute("id"), None);

    assert_eq!(div_element.get_attribute("itemscope"), Some(""));
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

#[test]
fn test_get_attribute_returns_the_raw_value() {
    let title = "<b>";
    let link = html! {
        <a title="Tom & Jerry" data-tag={title} data-quote="say \"hi\""></a>
    };
    let element = link.iter_elements().next().unwrap();

    assert_eq!(element.get_attribute("title"), Some("Tom & Jerry"));
    assert_eq!(element.get_attribute("data-tag"), Some("<b>"));
    assert_eq!(element.get_attribute("data-quote"), Some("say \"hi\""));
}

#[test]
fn test_set_attribute_value_is_escaped_when_rendered() {
    let mut link = html! { <a></a> };
    let element = link.iter_elements_mut().next().unwrap();

    element.set_attribute("title".to_owned(), "\"quoted\" & <b>".to_owned());

    assert_eq!(
        link.to_string(),
        r#"<a title="&quot;quoted&quot; &amp; &lt;b&gt;"></a>"#
    );
}

#[test]
fn test_get_then_set_attribute_does_not_change_the_output() {
    let mut link = html! { <a title="Tom & Jerry"></a> };
    let before = link.to_string();

    let element = link.iter_elements_mut().next().unwrap();
    let title = element.get_attribute("title").unwrap().to_owned();
    element.set_attribute("title".to_owned(), title);

    assert_eq!(link.to_string(), before);
}

#[test]
fn test_get_attribute_can_be_matched_on() {
    let inputs = html! {
        <input type="checkbox" />
        <input type="text" />
        <input />
    };

    let kinds: Vec<&str> = inputs
        .iter_elements()
        .map(|input| match input.get_attribute("type") {
            Some("checkbox") => "checkbox",
            Some(_) => "other",
            None => "missing",
        })
        .collect();

    assert_eq!(kinds, ["checkbox", "other", "missing"]);
}
