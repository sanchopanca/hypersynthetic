use hypersynthetic::prelude::*;

#[component]
fn Component(val1: &str, val2: i32) -> HtmlFragment {
    html! {
        <div>
            <p>{val1}</p>
            <p>{val2 + 1}</p>
        </div>
    }
}

mod inner {
    use hypersynthetic::prelude::*;
    #[component]
    pub fn InnerComponent(val: &str) -> HtmlFragment {
        html! {
            <span>{val}</span>
        }
    }

    #[component]
    pub fn Div(content: HtmlFragment) -> HtmlFragment {
        html! {
            <div>{{ content }}</div>
        }
    }
}

#[test]
fn test_component() {
    let result = html! {
        <Component val1="Hello" val2={41} />
    };

    let string_representation = result.to_string();

    let expected = "<div><p>Hello</p><p>42</p></div>";

    assert_eq!(string_representation, expected);
}

#[test]
fn test_component_as_a_child() {
    let result = html! {
        <div>
            <Component val1="test" val2={-1} />
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><div><p>test</p><p>0</p></div></div>";

    assert_eq!(string_representation, expected);
}

#[test]
fn test_inner_component() {
    let result = html! {
        <inner::InnerComponent val="test" />
    };

    let string_representation = result.to_string();

    let expected = "<span>test</span>";

    assert_eq!(string_representation, expected);
}

#[test]
fn test_inner_component_with_slot_argument() {
    let result = html! {
        <inner::Div>
            <inner::InnerComponent val="test" />
        </inner::Div>
    };

    let string_representation = result.to_string();

    let expected = "<div><span>test</span></div>";

    assert_eq!(string_representation, expected);
}

#[component]
fn NoArgsComponent() -> HtmlFragment {
    html! {
        <div></div>
    }
}

#[test]
fn test_no_args_component() {
    let result = html! {
        <NoArgsComponent />
    };

    let string_representation = result.to_string();

    let expected = "<div></div>";

    assert_eq!(string_representation, expected);
}

#[component]
fn Incremented(mut n: i32) -> HtmlFragment {
    n += 1;
    html! { <p>{n}</p> }
}

#[test]
fn test_mut_parameter() {
    let result = html! { <Incremented n={1} /> };

    assert_eq!(result.to_string(), "<p>2</p>");
}

#[component]
fn Appended(mut content: HtmlFragment, mut suffix: String) -> HtmlFragment {
    suffix.push('!');
    content.push(hypersynthetic::Node::Text(suffix));
    html! { <div>{{ content }}</div> }
}

#[test]
fn test_mut_parameters_with_slot() {
    let result = html! {
        <Appended suffix={"hi".to_owned()}><b>"x"</b></Appended>
    };

    assert_eq!(result.to_string(), "<div><b>x</b>hi!</div>");
}

// The compiler evaluates `#[cfg]` before running attribute macros, even when it
// comes after `#[component]`, so a disabled component disappears entirely.
#[component]
#[cfg(any())]
fn NeverCompiled() -> HtmlFragment {
    html! { <p>"never"</p> }
}

#[component]
#[cfg(test)]
fn AlwaysCompiled() -> HtmlFragment {
    html! { <p>"always"</p> }
}

#[test]
fn test_cfg_on_component() {
    let result = html! { <AlwaysCompiled /> };

    assert_eq!(result.to_string(), "<p>always</p>");
}

#[test]
fn test_component_paths_starting_with_keywords() {
    let result = html! {
        <self::inner::InnerComponent val="a" />
        <crate::inner::InnerComponent val="b" />
    };

    assert_eq!(result.to_string(), "<span>a</span><span>b</span>");
}

#[component]
fn Counter(
    #[builder(default)] count: i32,
    #[builder(default = "item".to_owned(), setter(into))] label: String,
) -> HtmlFragment {
    html! { <p>{label}": "{count}</p> }
}

#[test]
fn test_builder_attributes_on_props() {
    let result = html! {
        <Counter />
        <Counter count={3} label="apple" />
    };

    assert_eq!(result.to_string(), "<p>item: 0</p><p>apple: 3</p>");
}

#[component]
fn SubmitButton(disabled: bool, #[builder(default)] primary: bool) -> HtmlFragment {
    html! { <button disabled={disabled} data-primary={primary}></button> }
}

#[test]
fn test_boolean_prop_shorthand() {
    let result = html! {
        <SubmitButton disabled />
        <SubmitButton disabled={false} primary />
    };

    assert_eq!(
        result.to_string(),
        "<button disabled data-primary=\"false\"></button>\
         <button data-primary=\"true\"></button>"
    );
}

// What the `impl Trait` error suggests instead (see compile_fail/impl_trait_prop.rs)
#[component]
fn Shows<T: std::fmt::Display>(value: T) -> HtmlFragment {
    html! { <p>{value}</p> }
}

#[component]
fn List<T: std::fmt::Display>(items: Vec<T>) -> HtmlFragment {
    html! { <li :for={item in items}>{item}</li> }
}

#[test]
fn test_generic_props() {
    let result = html! {
        <Shows value={1} />
        <Shows value="two" />
        <List items={vec![3, 4]} />
        <List items={vec!["five"]} />
    };
    assert_eq!(
        result.to_string(),
        "<p>1</p><p>two</p><li>3</li><li>4</li><li>five</li>"
    );
}
