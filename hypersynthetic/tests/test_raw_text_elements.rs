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
