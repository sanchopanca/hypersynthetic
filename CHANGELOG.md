# Changelog

Changes to `hypersynthetic` and `hypersynthetic_macros`, which are released together.

## 0.10.0 (unreleased)

A large release: safer output, much faster building of pages that use components with
children, a DOM-like API for changing HTML after it's built, and many fixes to `html!`
and `#[component]`.

### Breaking changes

They come in two kinds: changes the compiler will point out when you upgrade, and
changes that compile as before but render different HTML. Read the second list
carefully.

#### The compiler will point these out

- Requires Rust 1.85 or newer (now declared as `rust-version`).
- **The content between a component's tags goes to the parameter named `children`.**
  In 0.9 it went to the first parameter of type `HtmlFragment`, whatever its name. Now
  `children` can be anywhere in the parameter list, and other `HtmlFragment` parameters
  are ordinary props, so a component can take several pieces of HTML. Rename your slot
  parameters:

  ```rust
  // 0.9
  #[component]
  fn Card(inner_block: HtmlFragment, title: &str) -> HtmlFragment { ... }

  // 0.10
  #[component]
  fn Card(children: HtmlFragment, title: &str) -> HtmlFragment { ... }
  ```

  `children` is optional (an empty fragment when no content is given), can be passed
  as an attribute (`<Card children={fragment} />`), and must be an `HtmlFragment`. If a
  component accepts children but doesn't use them, keep the name and add
  `#[allow(unused_variables)]` to the parameter: `_children` is rejected, since it
  wouldn't receive the content.
- Removed from the public API: the `ComponentWithSlots` and `PropsOrNoPropsBuilder`
  traits, and the `component_with_slots_view` and `component_with_slots_props_builder`
  functions. `component_view` and `component_props_builder` are no longer in the prelude
  and are hidden from the docs: they're only for code that `html!` generates.
- `typed_builder` and `typed_builder_macro` are no longer re-exported (at the crate root
  or in the prelude), so typed-builder's version isn't part of hypersynthetic's API.
  `#[builder(...)]` attributes on component parameters keep working without it; code
  that used `hypersynthetic::typed_builder` directly needs its own dependency.
- Inside `<script>` and `<style>`, `{value}` (also inside a string literal) is a compile
  error: HTML escaping can't make a value safe in JavaScript or CSS. Insert values with
  `{{value}}`, after making sure they're safe as code (e.g. JSON-encoded). Elements and
  components inside `<script>`/`<style>` are compile errors too.
- A lowercase path as a tag name (`<foo::bar>`) is a compile error; 0.9 rendered
  `<bar>`.
- `HtmlFragment` is a struct with a private field instead of the enum
  `HtmlFragment::Nodes(Vec<Node>)`. Build fragments with `HtmlFragment::new`,
  `From<Vec<Node>>` or `collect()`, and read them with `child_nodes()`, `iter()`,
  `into_nodes()`, ...
- Names, values and text are `Cow<'static, str>` instead of `String`: text written in a
  template is borrowed rather than copied on every render.
  - `ElementData::tag_name`, `Attribute::name`, `Attribute::value` (an
    `Option<Cow<'static, str>>`), and `Node::Text`.
  - `ElementData::new` and `set_attribute` accept `&'static str` or `String`, so most
    calls still compile.
  - Build text nodes with `Node::text(...)` (escapes) or `Node::raw_html(...)` (as is);
    `Node::Text(s.into())` also works. Read a `Cow` as a `String` with `.into_owned()`,
    or as a `&str` with `&*`.
- `ElementData::get_attribute` returns `Option<&str>` (was `Option<String>`), and an
  attribute without a value gives `Some("")`.

#### Compiles as before, renders differently

- `{fragment}` inserts an `HtmlFragment` as HTML. In 0.9 it was formatted and escaped,
  so the markup showed up as text; `{{fragment}}` produced a single raw text node and
  now inserts the nodes too.
- A `bool` value on an HTML boolean attribute (`disabled`, `checked`, `selected`,
  `hidden`, `required`, ... 28 in all, per the HTML spec) decides whether the attribute
  is there: `disabled={false}` leaves it out, `disabled={true}` renders `disabled`. In
  0.9 it rendered `disabled="false"`, which still disabled the element. On other
  attributes a `bool` is still `"true"`/`"false"` (`aria-expanded`, `hx-boost`).
- String literals inside `<script>` and `<style>` are no longer HTML-escaped (`a > b` in
  CSS and `&&` in JavaScript were broken).
- Inside `<script>` and `<style>`, rendering escapes sequences that would end the
  element early: the `s` of `<script`/`</script` becomes `\u0073` (or `\u0053`), and
  the `s` of `</style` becomes `\73` (or `\53`), as React does. These mean the same in
  JavaScript and CSS strings, so `{{value}}` in a script can't close it.
- `<div />` and other self-closing non-void elements render as `<div></div>`. Browsers
  ignore `/>` on them, so 0.9's output opened an element that swallowed its following
  siblings. Void elements (`<br />`, `<img />`, ...) are unchanged.
- An attribute whose name isn't valid HTML (empty, or containing whitespace, quotes,
  `>`, `/`, `=`, `<` or control characters) is left out when rendering, and an element
  whose tag name isn't valid is left out with its children. Dynamic attribute names
  (`<div {name}="v">`) were escaped in 0.9, which didn't prevent injecting attributes.
- `{{` and `}}` are literal braces in attribute values, as in text and `format!`: write
  `x-data="{{ open: false }}"` for `x-data="{ open: false }"`.
- Attribute values are stored unescaped and escaped when rendering. Values passed to
  `set_attribute` are now escaped, so don't escape them beforehand, and `get_attribute`
  returns the value as written (0.9 returned `&amp;` for values from `html!`).
- `set_attribute` replaces an existing attribute, keeping its position and removing
  duplicates of it, like the DOM's `setAttribute()`. In 0.9 it always appended.

### Added

#### `html!`

- `Option` values: `{option}` renders the value inside `Some` and nothing for `None`, and
  `attr={option}` leaves the attribute out for `None`.
- Any expression, with an optional format spec, inside string literals, in text and in
  attribute values: `"{item.name}: {item.price:.2}"`.
- Custom element names with hyphens (`<my-widget>`) and element names that are Rust
  keywords (`<use>` in SVG).
- Alpine.js- and htmx-style attribute names: `@click`, `:class`,
  `x-on:submit.prevent`, `x-on:input.debounce.500ms`, `data-2`.

#### Components

- `#[builder(...)]` on parameters configures the prop: `#[builder(default)]` makes it
  optional, `#[builder(setter(into))]` accepts anything that converts into its type.
- Boolean prop shorthand: `<Button disabled />` is `<Button disabled={true} />`.
- Parameters can use `mut` and other binding modes (`mut items: Vec<String>`).
- Doc comments and `#[deprecated]` on a component apply to the component users call,
  and its `Props` struct gets a doc comment, so components work with
  `#![deny(missing_docs)]`.
- Generic props (`fn List<T: Display>(items: Vec<T>)`) and props with elided lifetimes
  in any form (`&str`, `&'_ T`, `Cow<'_, str>`).

#### A DOM-like API

For changing HTML after it's built, with the DOM's names in snake_case. The docs of
`HtmlFragment` have a table mapping DOM operations to these.

- `child_nodes()` (a slice) and `child_nodes_mut()` (a `Vec`, whose methods cover
  `insertBefore`, `prepend`, `removeChild` and `replaceChildren`), on fragments and
  elements.
- `class_list()` and `class_list_mut()`: the DOM's `classList`, with `contains`, `add`,
  `remove`, `toggle` and `replace` (types `ClassList` and `ClassListMut`).
- `ElementData::toggle_attribute`, `get_attribute_names`, `id` and `set_text_content`.
- Searching, on fragments and elements: `descendants()` (every element below, in tree
  order), `get_element_by_id`, `get_elements_by_tag_name`, `get_elements_by_class_name`,
  and for changing elements, `get_element_by_id_mut` and `for_each_descendant_mut`.
- `Node::text` (escapes, like `createTextNode()`) and `Node::raw_html` (as is).

#### `HtmlFragment`

- Collecting fragments into one: `items.iter().map(|item| html! { ... }).collect()`.
- `PartialEq` and `Eq` (also on `Node`, `ElementData` and `Attribute`), `Default`,
  `From<Vec<Node>>`, `FromIterator<Node>`, `Extend<Node>`, `IntoIterator` (owned, `&`
  and `&mut`), `len`, `is_empty` and `into_nodes`.

### Deprecated

- `HtmlFragment::get_nodes()`, which always copied. Use `child_nodes()` to read the nodes,
  `child_nodes().to_vec()` for a copy, or `into_nodes()` if the fragment isn't needed
  anymore.

### Fixed

- Security: dynamic attribute names could inject attributes
  (`name = "x onmouseover=alert(1)"`), and a `"` in a value passed to `set_attribute`
  could end the attribute. Both are now prevented (see above).
- Variables named like `html!`'s internal ones (`let v = ...; html! { {v} }`) no longer
  conflict with it.
- A local item named `hypersynthetic` no longer breaks `html!` and `#[component]`.
- Literal text between two interpolations was dropped (`"{a}xyz{b}"`).
- `<x crate true false>` rendered one attribute named `cratetruefalse`.
- `#[builder(default)]` failed in crates that don't depend on typed-builder.
- Compile errors instead of panics ("proc macro panicked") for hyphenated prop names
  (`<C data-id="1" />`, with a suggestion), dynamic prop names and `#[component]` on
  something that isn't a function.
- Clearer compile errors: for `<!DOCTYPE html>` typos, unclosed tags (pointing at the
  opening tag), mismatched closing tags (with both names), unexpected tokens among
  children, repeated `:if` or `:for`, `#[component]` with arguments, `impl Trait` props
  (one error suggesting a generic parameter, instead of several about generated code),
  and prop parameters that aren't names.
- The Rocket responder uses `RawHtml`; responses are unchanged.

### Performance

- Rendering writes straight into the output instead of building a string per level:
  3.3× faster for a 100-row table page.
- Fragments are copy-on-write, so inserting one (`{children}` in a layout component)
  shares its nodes instead of deep-copying them at every level of nesting. Building a
  page with three levels of layout components went from 586 µs to 193 µs (with the
  changes below).
- Tag names, attribute names, literal attribute values and literal text from templates
  are borrowed instead of allocated on every render: building pages 15–41% faster,
  depending on the page.
- Tag and attribute names are validated with a byte lookup table.
