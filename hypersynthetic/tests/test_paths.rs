// A local item named `hypersynthetic` must not break the generated code,
// which should always refer to the crate as `::hypersynthetic`.
mod hypersynthetic {}

use ::hypersynthetic::prelude::*;

#[component]
fn NoProps() -> HtmlFragment {
    html! { <hr /> }
}

#[component]
fn WithProps(text: &str) -> HtmlFragment {
    html! { <p>{text}</p> }
}

#[component]
fn WithSlot(children: HtmlFragment) -> HtmlFragment {
    html! { <div>{{ children }}</div> }
}

#[test]
fn test_generated_code_ignores_local_module_named_hypersynthetic() {
    let id = 7;
    let raw = "<b>raw</b>";
    let result = html! {
        <!DOCTYPE html>
        <p class="literal" id={id} data-x="x{id}" hidden>"text {id}"</p>
        <i :for={n in 0..2}>{n}</i>
        <u :if={id > 0}>{{ raw }}</u>
        <NoProps />
        <WithProps text="a" />
        <WithProps :for={t in ["b", "c"]} text={t} />
        <WithSlot><span>"slot"</span></WithSlot>
    };

    assert_eq!(
        result.to_string(),
        "<!DOCTYPE html>\
         <p class=\"literal\" id=\"7\" data-x=\"x7\" hidden>text 7</p>\
         <i>0</i><i>1</i>\
         <u><b>raw</b></u>\
         <hr />\
         <p>a</p><p>b</p><p>c</p>\
         <div><span>slot</span></div>"
    );
}
