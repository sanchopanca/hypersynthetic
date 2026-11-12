use hypersynthetic::prelude::*;

#[test]
fn test_for_attribute() {
    let result = html! {
        <form hx-post="/users">
            <label for="email">"Email"</label>
            <input type="email" id="email" name="email" required />
        </form>
    };

    assert_eq!(
        result.to_string(),
        "<form hx-post=\"/users\"><label for=\"email\">Email</label><input type=\"email\" id=\"email\" name=\"email\" required /></form>"
    );
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

#[test]
fn test_keyword_attribute_names() {
    let result = html! {
        <link rel="preload" as="style" />
        <script async type="module"></script>
        <video loop muted></video>
        <label for="name"></label>
        <feBlend in="SourceGraphic" />
    };

    assert_eq!(
        result.to_string(),
        "<link rel=\"preload\" as=\"style\" />\
         <script async type=\"module\"></script>\
         <video loop muted></video>\
         <label for=\"name\"></label>\
         <feBlend in=\"SourceGraphic\"></feBlend>"
    );
}

#[test]
fn test_newer_keywords_and_booleans_as_attribute_names() {
    let result = html! {
        <x-el gen="1" self crate true false></x-el>
    };

    assert_eq!(
        result.to_string(),
        "<x-el gen=\"1\" self crate true false></x-el>"
    );
}

// Props can be named after keywords too. The parameter has to be a raw identifier
// (`r#type`); in `html!` it's written like the HTML attribute.

#[component]
fn Field(r#type: &str, r#for: &str, #[builder(default)] r#loop: bool) -> HtmlFragment {
    html! {
        <input type={r#type} form={r#for} data-loop={r#loop} />
    }
}

#[test]
fn test_keyword_prop_names() {
    let result = html! {
        <Field type="email" for="signup" />
        <Field type="text" for="login" loop />
    };

    assert_eq!(
        result.to_string(),
        "<input type=\"email\" form=\"signup\" data-loop=\"false\" />\
         <input type=\"text\" form=\"login\" data-loop=\"true\" />"
    );
}

// `gen` is only a keyword since the 2024 edition
#[component]
fn Sequence(r#gen: u32, r#try: u32) -> HtmlFragment {
    html! { <i>{r#gen}"-"{r#try}</i> }
}

#[test]
fn test_newer_keywords_as_prop_names() {
    let result = html! {
        <Sequence gen={3} try={1} />
    };

    assert_eq!(result.to_string(), "<i>3-1</i>");
}
