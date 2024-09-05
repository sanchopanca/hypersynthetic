use hypersynthetic::prelude::*;

#[test]
fn test_required_textarea() {
    let result = html! {
        <textarea required></textarea>
    };

    assert_eq!(result.to_string(), "<textarea required></textarea>");
}
