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
fn test_component_arguments_out_of_order() {
    let result = html! {
        <Component val2={41} val1="Hello" />
    };

    let string_representation = result.to_string();

    let expected = "<div><p>Hello</p><p>42</p></div>";
    assert_eq!(string_representation, expected);
}

// #[component]
// fn Component(val1: &str, val2: &str) -> HtmlFragment {
//     html! {
//         <div>
//             <p>{val1}</p>
//             <p>{val2}</p>
//         </div>
//     }
// }

// struct ComponentArgs<'a> {
//     val1: &'a str,
//     val2: &'a str,
// }

// fn ComponentCompanion(ComponentArgs { val1, val2 }: ComponentArgs) -> NodeCollection {
//     Component(val1, val2)
// }

// #[test]
// fn test_component_arguments_out_of_order() {
//     {
//         let first = "first".to_ascii_uppercase();
//         {
//             let second = "SECOND".to_ascii_lowercase();
//             ComponentCompanion(ComponentArgs {
//                 val1: &first,
//                 val2: &second,
//             });
//         }
//     }
//     // let result = html! {
//     //     <Component val2={41} val1={"Hello".to_owned()} />
//     // };

//     // let string_representation = result.to_string();

//     // let expected = "<div><p>Hello</p><p>42</p></div>";
//     // assert_eq!(string_representation, expected);
// }
