use hypersynthetic::prelude::*;

#[test]
fn test_for_attribute() {
    let result = html! {
        <form hx-post="/users">
            <label for="email">"Email"</label>
            <input type="email" id="email" name="email" required />
        </form>
    };

    assert_eq!(result.to_string(),
        "<form hx-post=\"/users\"><label for=\"email\">Email</label><input type=\"email\" id=\"email\" name=\"email\" required /></form>");
}

#[test]
fn test_keyword_attr_with_dashes() {
    let result = html! {
        <div what-if-this="works">"Hello!"</div>
    };

    assert_eq!(
        result.to_string(),
        "<div what-if-this=\"works\">Hello!</div>"
    );
}

#[test]
fn test_async_attribute() {
    let result = html! {
        <div async="on">"Hello!"</div>
    };

    assert_eq!(result.to_string(), "<div async=\"on\">Hello!</div>");
}

#[test]
fn test_yield_attribute() {
    let result = html! {
        <div yield="on">"Hello!"</div>
    };

    assert_eq!(result.to_string(), "<div yield=\"on\">Hello!</div>");
}

#[test]
fn test_try_attribute() {
    let result = html! {
        <div try="once">"Hello!"</div>
    };

    assert_eq!(result.to_string(), "<div try=\"once\">Hello!</div>");
}

#[test]
fn test_union_attribute() {
    let result = html! {
        <div union="on">"Hello!"</div>
    };

    assert_eq!(result.to_string(), "<div union=\"on\">Hello!</div>");
}

#[test]
fn test_boolean_literal_attribute() {
    let result = html! {
        <div true="on" false="off">"Hello!"</div>
    };

    assert_eq!(
        result.to_string(),
        "<div true=\"on\" false=\"off\">Hello!</div>"
    );
}
