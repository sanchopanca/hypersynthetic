# hypersynthetic

[![](https://badgers.space/badge/crates.io/hypersynthetic)](https://crates.io/crates/hypersynthetic)
[![](https://badgers.space/github/checks/sanchopanca/hypersynthetic)](https://github.com/sanchopanca/hypersynthetic/actions)

An HTML template engine that chose composition over inheritance

# Example

```rust
use hypersynthetic::prelude::*;

#[component]
fn TodoItem(text: &str, done: bool) -> NodeCollection {
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

    let rendered_list = html! {
        <ul>
            <TodoItem :for={(text, done) in todo_list} text={text} done={done} />
        </ul>
    };

    // ... Render `rendered_list` into your application.
}
```

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