# hypersynthetic

[![](https://badgers.space/badge/crates.io/hypersynthetic)](https://crates.io/crates/hypersynthetic)
[![](https://badgers.space/github/checks/sanchopanca/hypersynthetic)](https://github.com/sanchopanca/hypersynthetic/actions)
[![](https://badgers.space/badge/%E2%80%8B/docs.rs/orange?icon=eva-book-open-outline)](https://docs.rs/hypersynthetic/latest/hypersynthetic/index.html)

Hypersynthetic is a library for writing HTML inside Rust.
It is inspired by JSX and HEEx templates, and tries to be different from Tera and Minijinja in one key aspect: it only allows reusing HTML code via composition not via inheritance.
It is suitable for building traditional web applications, where backend responds with HTML.

Here is an example of what hypersynthetic can do:

# Example

```rust
use hypersynthetic::prelude::*;

#[component]
fn TodoItem(text: &str, done: bool) -> HtmlFragment {
    let text_decoration = if done { "line-through" } else { "none" };

    html! {
        <li style="text-decoration: {text_decoration};">
            {text}
        </li>
    }
}

fn main() {
    let todo_list = vec![
        ("Buy Milk", true),
        ("Read Rust Book", false),
        ("Write Web App using html! macro", false),
    ];

    let html_list = html! {
        <ul>
            <TodoItem :for={(text, done) in todo_list} text={text} done={done} />
        </ul>
    };

    // ... Render `html_list` into your application.
    html_list.to_string();
}
```

In this example:

The `TodoItem` component displays a to-do item, striking it through if it’s done.
The main function defines a list of to-dos and uses the `:for` attribute to loop over them, rendering each one using the `TodoItem` component.

See the [html](https://docs.rs/hypersynthetic/latest/hypersynthetic/macro.html.html) macro for the description of the syntax
and [component](https://docs.rs/hypersynthetic/latest/hypersynthetic/attr.component.html) macro for more details about using components

## Features
The following features enable integration with popular web frameworks. See [the Cargo Book](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features) to learn more about enabling features.

- `rocket`: Enables integration with the Rocket web framework and allows to return `HtmlFragment` from handlers.
- `axum`: Enables integration with the Axum web framework and allows to return `HtmlFragment` from handlers.


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
