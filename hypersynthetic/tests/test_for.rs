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

#[test]
fn test_for() {
    let numbers = [1, 2, 3];
    let result = html! {
        <div>
            <p :for={number in numbers}>
                { number }
            </p>
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><p>1</p><p>2</p><p>3</p></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_deep() {
    let things = [(1, 2), (3, 4), (5, 6)];
    let result = html! {
        <tr :for={thing in things}>
            <td>{ thing.0 }</td>
            <td>{ thing.1 }</td>
        </tr>
    };

    let string_representation = result.to_string();

    let expected =
        "<tr><td>1</td><td>2</td></tr><tr><td>3</td><td>4</td></tr><tr><td>5</td><td>6</td></tr>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_selfclosing() {
    let numbers = [1, 2, 3];
    let result = html! {
        <input type="text" :for={number in numbers} id="id{number}" />
    };

    let string_representation = result.to_string();

    let expected = "<input type=\"text\" id=\"id1\" /><input type=\"text\" id=\"id2\" /><input type=\"text\" id=\"id3\" />";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_deep_components() {
    let numbers = [1, 2];
    let result = html! {
        <div :for={number in numbers} id="id{number}">
            <Component val1="test" val2={number} />
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div id=\"id1\"><div><p>test</p><p>2</p></div></div><div id=\"id2\"><div><p>test</p><p>3</p></div></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_on_a_component() {
    let numbers = [1, 2];
    let result = html! {
        <Component :for={number in numbers} val1="test" val2={number} />
    };

    let string_representation = result.to_string();

    let expected = "<div><p>test</p><p>2</p></div><div><p>test</p><p>3</p></div>";
    assert_eq!(string_representation, expected);
}

#[test]
fn test_for_parentheses() {
    let numbers = [(1, "one"), (2, "two)")];
    let result = html! {
        <div :for={(number, name) in numbers}>
            <p>{ number }</p>
            <p>{ name }</p>
        </div>
    };

    let string_representation = result.to_string();

    let expected = "<div><p>1</p><p>one</p></div><div><p>2</p><p>two)</p></div>";
    assert_eq!(string_representation, expected);
}
