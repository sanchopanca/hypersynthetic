//! `<script>` and `<style>` content isn't HTML: browsers don't decode entities there,
//! so their text is rendered as written, without HTML escaping.

use hypersynthetic::html;

#[test]
fn test_style_text_is_not_escaped() {
    let result = html! { <style>"a > b {{ color: red }}"</style> };
    assert_eq!(result.to_string(), "<style>a > b { color: red }</style>");
}

#[test]
fn test_script_text_is_not_escaped() {
    let result = html! { <script>"if (a && b < c) {{ go(); }}"</script> };
    assert_eq!(
        result.to_string(),
        "<script>if (a && b < c) { go(); }</script>"
    );
}

#[test]
fn test_unescaped_values_in_script() {
    let count = 5;
    let result = html! { <script>"let n = "{{count}}";"</script> };
    assert_eq!(result.to_string(), "<script>let n = 5;</script>");
}

#[test]
fn test_other_elements_are_still_escaped() {
    // Browsers decode entities in <textarea> and <title>, so escaping is right there
    let result = html! {
        <title>"a < b"</title>
        <textarea>"a & b"</textarea>
    };
    assert_eq!(
        result.to_string(),
        "<title>a &lt; b</title><textarea>a &amp; b</textarea>"
    );
}

// When rendering, a `<script>`'s content can't contain `<script` or `</script`, and a
// `<style>`'s can't contain `</style`: the element would end early, or (for
// `<!--<script>`) the browser would miss its real end. Like React, the `s` is replaced
// by the language's own escape, which means the same in a string or a regex.

#[test]
fn test_script_cant_be_closed_by_a_value() {
    let value = "</script><script>alert(1)//";
    let result = html! { <script>"let v = \""{{value}}"\";"</script> };

    assert_eq!(
        result.to_string(),
        r#"<script>let v = "</\u0073cript><\u0073cript>alert(1)//";</script>"#
    );
}

#[test]
fn test_script_guard_ignores_case_and_keeps_it() {
    let value = "</SCRIPT> <!--<Script>";
    let result = html! { <script>{{value}}</script> };

    assert_eq!(
        result.to_string(),
        r#"<script></\u0053CRIPT> <!--<\u0053cript></script>"#
    );
}

#[test]
fn test_style_cant_be_closed_by_a_value() {
    let value = "a::after { content: \"</style><style>\" }";
    let result = html! { <style>{{value}}</style> };

    // Only the closing tag ends a style; `<style` in it is harmless
    assert_eq!(
        result.to_string(),
        r#"<style>a::after { content: "</\73tyle><style>" }</style>"#
    );
}

#[test]
fn test_script_guard_leaves_ordinary_code_alone() {
    let code = "if (a < b && c <s) { el.innerHTML = '<span>' + '<scripts>'; }";
    let result = html! { <script>{{code}}</script> };

    assert_eq!(
        result.to_string(),
        r#"<script>if (a < b && c <s) { el.innerHTML = '<span>' + '<\u0073cripts>'; }</script>"#
    );
}

#[test]
fn test_script_guard_covers_elements_built_by_hand() {
    use hypersynthetic::{ElementData, Node};

    let mut script = ElementData::new("SCRIPT");
    script.add_child(Node::raw_html("let a = 1;"));
    script.add_child(Node::Element(ElementData::new("script")));

    assert_eq!(
        Node::Element(script).to_string(),
        r"<SCRIPT>let a = 1;<\u0073cript></\u0073cript></SCRIPT>"
    );
}

#[test]
fn test_guard_only_applies_to_script_and_style() {
    let value = "</script>";
    let result = html! { <p>{{value}}</p><textarea>{{value}}</textarea> };

    assert_eq!(
        result.to_string(),
        "<p></script></p><textarea></script></textarea>"
    );
}
