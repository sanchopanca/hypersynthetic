use hypersynthetic_macros::html;

#[test]
fn test_basic_html_macro() {
    let result = html! {
        <body>
            <div>
                <a>"Link"</a>
            </div>
        </body>
    };

    let string_representation = result.to_html();

    let expected = "<body><div><a>Link</a></div></body>";
    assert_eq!(string_representation, expected);
}
