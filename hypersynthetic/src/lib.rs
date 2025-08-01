//! # About Hypersynthetic
//!
//! Hypersynthetic is a library for writing HTML inside Rust.
//! It is inspired by JSX and HEEx templates, and tries to be different from Tera and Minijinja
//! by enabling [Locality of Behavior (LoB)](https://htmx.org/essays/locality-of-behaviour/)
//! and only allowing reusing HTML code via composition and not via inheritance.
//! It is suitable for building traditional web applications, where backend responds with HTML.
//!
//! Here is an example of what hypersynthetic can do:
//!
//! ```
//! use hypersynthetic::prelude::*;
//!
//! #[component]
//! fn TodoItem(text: &str, done: bool) -> HtmlFragment {
//!     let text_decoration = if done { "line-through" } else { "none" };
//!
//!     html! {
//!         <li style="text-decoration: {text_decoration};">
//!             {text}
//!         </li>
//!     }
//! }
//!
//! fn main() {
//!     let todo_list = vec![
//!         ("Buy Milk", true),
//!         ("Read Rust Book", false),
//!         ("Write Web App using html! macro", false),
//!     ];
//!
//!     let html_list = html! {
//!         <ul>
//!             <TodoItem :for={(text, done) in todo_list} text={text} done={done} />
//!         </ul>
//!     };
//!
//!     // ... Render `html_list` into your application.
//!     assert_eq!(html_list.to_string(),  "\
//!         <ul>\
//!             <li style=\"text-decoration: line-through;\">Buy Milk</li>\
//!             <li style=\"text-decoration: none;\">Read Rust Book</li>\
//!             <li style=\"text-decoration: none;\">Write Web App using html! macro</li>\
//!         </ul>"
//!     );
//! }
//! ```
//!
//! In this example:
//!
//! The TodoItem component displays a to-do item, striking it through if it's done.
//! The main function defines a list of to-dos and uses the :for attribute to loop over them,
//! rendering each one using the TodoItem component.
//!
//! See the [html] macro for the description of the syntax and [component] macro for more details about using components
//!
//! ## Features
//!
//! - `rocket`: Enables integration with the Rocket web framework. It allows to return [HtmlFragment] from the route handlers and sets the response content type to `text/html`.
//! - `axum`: Enables integration with the Axum web framework. It allows to return [HtmlFragment] from the route handlers and sets the response content type to `text/html`.

pub use htmlize::{escape_attribute, escape_text};
pub use typed_builder;
pub use typed_builder_macro;

pub mod component;

/// The component macro provides a way to define reusable and self-contained web components.
/// A component is a function that returns a [HtmlFragment]. The easiest way to create one is
/// [html] macro.
///
/// # Basic Usage
///
/// ```
/// # use hypersynthetic::prelude::*;
/// #[component]
/// fn MyComponent(prop1: &str, prop2: i32) -> HtmlFragment {
///    html! {
///        <div>
///            <p>{prop1}</p>
///            <p>{prop2}</p>
///        </div>
///    }
/// }
///
/// // ...
///
/// # fn main() {
/// let html = html! {
///     <MyComponent prop1="Hello" prop2={42} />
/// };
/// # }
/// ```
///
/// A component name must start with an uppercase letter and it must return a [HtmlFragment].
///
/// # Props
/// Components accept properties, similar to function arguments.
/// These properties can be any Rust type, providing a type-safe way to pass data to components.
/// When calling a component from [html] macro, the properties are passed as html arguments.
/// The arguments can be in any order, it's not necessary for them to be in the same order as defined in the component function.
///
/// # Slots
///
/// Components in this library can accept a slot argument, which allows for flexible and reusable HTML structures.
/// The slot argument must be an [HtmlFragment] and should be the first argument in the component function.
///
/// ## Slot Example
///
/// Here is how to create and use a component called `OrangeDiv`
/// that wraps its content in a styled `<div>` element:
///
/// ```
/// # use hypersynthetic::prelude::*;
/// #[component]
/// fn OrangeDiv(inner_block: HtmlFragment) -> HtmlFragment {
///     html! {
///         <div class="orange round">
///             {{ inner_block }}
///         </div>
///     }
/// }
///
/// // ...
///
/// # fn main() {
/// let data = "Hello, world!";
/// let result = html! {
///     <OrangeDiv>
///         <p>{ data }</p>
///     </OrangeDiv>
/// };
///
/// assert_eq!(
///     result.to_string(),
///     r#"<div class="orange round"><p>Hello, world!</p></div>"#
/// );
/// # }
/// ```
///
/// In the `OrangeDiv` component, `inner_block` represents the slot content
/// that will be injected into the `<div>` element.
/// The double curly braces `{{ }}` are used to disable HTML escaping,
/// which is the desired behavior in most cases to ensure the HTML content is rendered correctly.
pub use hypersynthetic_macros::component;

/// The `html` macro allows to construct html fragments in Rust.
/// It mostly follows the same syntax as HTML with the following exceptions:
///
/// 1. Void tags need to be self-closing. For example `<br>` should be written as `<br />`.
///
/// 2. Bare text is not (yet) allowed. Instead you should put a string literal inside an expression
///    like `<span>{"text"}</span>`. Alternatively, for convenience, you can use the string literal
///    directly like `<span>"text"</span>`. But the latter syntax is likely to change in the future.
///    See this [github issue](https://github.com/sanchopanca/hypersynthetic/issues/3) for details.
///
/// With gotchas out the way, here are the features:
///
/// # Dynamic content
/// An expression that implements Display trait (or just ToString trait) inside curly braces (`{expression}`)
/// will be substituted with the result of .to_string() call, applying html escaping.
/// To avoid escaping, wrap the expression in double curly braces: `{{expression}}` (not available in string literals,
/// see an example below).
/// Here are the places where it can be used:
/// 1. As a child of an element.
/// ```
/// # use hypersynthetic::html;
/// let txt = "Text";
/// let div = html! {
///     <div>{txt}</div>
/// };
/// assert_eq!(div.to_string(), "<div>Text</div>");
///
/// let span = html! {
///     <span>{txt}{txt}</span>
/// };
/// assert_eq!(span.to_string(), "<span>TextText</span>");
/// ```
///
/// 2. In an attribute value or its part.
/// ```
/// # use hypersynthetic::html;
/// let cls = "header";
/// let div = html! {
///     <div class={cls}>{"Breaking news"}</div>
/// };
/// assert_eq!(div.to_string(), r#"<div class="header">Breaking news</div>"#);
///
/// let id = 42;
/// let span = html! {
///     <span id="header-{id}">{"Breaking news"}</span>
/// };
///
/// assert_eq!(span.to_string(), r#"<span id="header-42">Breaking news</span>"#);
/// ```
///
/// 3. In an attribute name.
/// ```
/// # use hypersynthetic::html;
/// let method = "hx-get";
/// let div = html! {
///     <button {method}="/resources">{"Get 'em"}</button>
/// };
/// assert_eq!(div.to_string(), r#"<button hx-get="/resources">Get 'em</button>"#);
/// ```
///  
/// 4. In a string literal.
/// ```
/// # use hypersynthetic::html;
/// let txt = "Hello";
/// let div = html! {
///     <div>"{txt} World"</div>
/// };
/// assert_eq!(div.to_string(), "<div>Hello World</div>");
/// ```
///
/// ## Disabling escaping
/// To disable escaping, use double curly braces: `{{expression}}`.
/// ```
/// # use hypersynthetic::html;
/// let txt = "<span>I know what I'm doing</span>";
/// let div = html! {
///     <div>{{txt}}</div>
/// };
/// assert_eq!(div.to_string(), "<div><span>I know what I'm doing</span></div>");
/// ```
/// ### Disabling escaping doesn't work in string literals
/// Under the hood, hypersynthetic uses Rust built-in `format!` function which uses {{ and }} to escape { and }.
/// So this is what happens if you use double curly braces in a string literal:
/// ```
/// # use hypersynthetic::html;
/// let txt = "Hello";
/// let div = html! {
///     <div>"{{txt}} World"</div>
/// };
/// assert_eq!(div.to_string(), "<div>{txt} World</div>");
/// ```
///
/// # Conditionals
/// A special pseudo-attribute `:if` is used to conditionally render an element.
/// ```
/// # use hypersynthetic::html;
/// let notify_user = true;
/// let div = html! {
///     <div :if={notify_user}>
///         <span>"You have been warned"</span>
///     </div>
/// };
/// assert_eq!(div.to_string(), "<div><span>You have been warned</span></div>");
///
/// let notify_user = false;
/// let div = html! {
///     <div :if={notify_user}>
///         <span>"You have been warned"</span>
///     </div>
/// };
/// assert_eq!(div.to_string(), "");
/// ```
///
/// To emulate `else` branch, reverse the condition:
/// ```
/// # use hypersynthetic::html;
/// let notify_user = false;
/// let div = html! {
///     <div :if={notify_user}>
///         <span>"Be careful"</span>
///     </div>
///     <div :if={!notify_user}>
///         <span>"You are safe"</span>
///     </div>
/// };
/// assert_eq!(div.to_string(), "<div><span>You are safe</span></div>");
/// ```
///
/// # Iteration
/// A special pseudo-attribute `:for` is used to iterate over an iterable object and create an element for each item.
/// ```
/// # use hypersynthetic::html;
/// let numbers = [1, 2];
/// let div = html! {
///     <div :for={n in numbers}>
///         <span>{n}</span>
///     </div>
/// };
/// assert_eq!(div.to_string(), "<div><span>1</span></div><div><span>2</span></div>");
///
/// let div = html! {
///     <input :for={n in numbers} type="text" value={n} />
/// };
/// assert_eq!(div.to_string(), r#"<input type="text" value="1" /><input type="text" value="2" />"#);
/// ```
///
/// # Components
/// Components can be called as tags. Here is an example:
/// ```
/// # use hypersynthetic::prelude::*;
/// #[component]
/// fn MyDiv(text: &str) -> HtmlFragment {
///     html! {
///         <div>{text}</div>
///     }
/// }
/// fn main() {
///     let div = html! {
///         <div>
///             <MyDiv text="my text" />
///         </div>
///     };    
/// }
/// ```
/// See [component] macro for more details.
pub use hypersynthetic_macros::html;

pub mod prelude {
    pub use crate::HtmlFragment;
    pub use crate::component::{Component, Props, component_props_builder, component_view};
    pub use crate::typed_builder;
    pub use crate::{component, html};
}

use std::fmt;
use std::slice::Iter;
use std::slice::IterMut;

#[derive(Clone, Debug)]
pub enum HtmlFragment {
    Nodes(Vec<Node>),
}

#[derive(Clone, Debug)]
pub enum Node {
    Element(ElementData),
    Text(String),
    DocType,
}

#[derive(Clone, Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub children: HtmlFragment,
    pub self_closing: bool,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl HtmlFragment {
    // TODO: consider something else except Vec
    pub fn new(nodes: Vec<Node>) -> Self {
        HtmlFragment::Nodes(nodes)
    }

    pub fn push(&mut self, node: Node) {
        match self {
            HtmlFragment::Nodes(nodes) => nodes.push(node),
        }
    }

    fn to_html(&self) -> String {
        match self {
            HtmlFragment::Nodes(nodes) => nodes.iter().map(|node| node.to_html()).collect(),
        }
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        match self {
            HtmlFragment::Nodes(nodes) => nodes.clone(),
        }
    }

    pub fn iter(&self) -> Iter<Node> {
        match self {
            HtmlFragment::Nodes(nodes) => nodes.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<Node> {
        match self {
            HtmlFragment::Nodes(nodes) => nodes.iter_mut(),
        }
    }

    pub fn iter_elements(&self) -> ElementDataIter {
        ElementDataIter { iter: self.iter() }
    }

    pub fn iter_elements_mut(&mut self) -> ElementDataIterMut {
        ElementDataIterMut {
            iter: self.iter_mut(),
        }
    }
}

impl<'a> IntoIterator for &'a HtmlFragment {
    type Item = &'a Node;
    type IntoIter = Iter<'a, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut HtmlFragment {
    type Item = &'a mut Node;
    type IntoIter = IterMut<'a, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Node {
    fn to_html(&self) -> String {
        match self {
            Node::Text(text) => text.clone(),
            Node::Element(element_data) => element_data.to_html(),
            Node::DocType => "<!DOCTYPE html>".to_owned(),
        }
    }
}

impl ElementData {
    pub fn new(tag_name: String) -> Self {
        ElementData {
            tag_name,
            attributes: Vec::new(),
            children: HtmlFragment::new(Vec::new()),
            self_closing: false,
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.iter().any(|attr| attr.name == name)
    }

    pub fn set_attribute(&mut self, name: String, value: String) {
        self.attributes.push(Attribute {
            name,
            value: Some(value),
        });
    }

    pub fn remove_attribute(&mut self, name: &str) {
        self.attributes.retain(|attr| attr.name != name);
    }

    pub fn get_attribute(&self, name: &str) -> Option<String> {
        self.attributes
            .iter()
            .find(|attr| attr.name == name)
            .map(|attr| attr.value.clone().unwrap_or("".to_owned()))
    }

    fn to_html(&self) -> String {
        let attributes_string: String = self
            .attributes
            .iter()
            .map(|attr| match &attr.value {
                Some(value) => format!(" {}=\"{}\"", attr.name, value),
                None => format!(" {}", attr.name),
            })
            .collect();

        let children_string: String = match &self.children {
            HtmlFragment::Nodes(nodes) => nodes.iter().map(|node| node.to_html()).collect(),
        };

        if self.self_closing {
            format!("<{}{} />", self.tag_name, attributes_string)
        } else {
            format!(
                "<{}{}>{}</{}>",
                self.tag_name, attributes_string, children_string, self.tag_name
            )
        }
    }
}

pub struct ElementDataIter<'a> {
    iter: Iter<'a, Node>,
}

impl<'a> Iterator for ElementDataIter<'a> {
    type Item = &'a ElementData;

    fn next(&mut self) -> Option<Self::Item> {
        for node in self.iter.by_ref() {
            if let Node::Element(element_data) = node {
                return Some(element_data);
            }
        }
        None
    }
}

pub struct ElementDataIterMut<'a> {
    iter: IterMut<'a, Node>,
}

impl<'a> Iterator for ElementDataIterMut<'a> {
    type Item = &'a mut ElementData;

    fn next(&mut self) -> Option<Self::Item> {
        for node in self.iter.by_ref() {
            if let Node::Element(element_data) = node {
                return Some(element_data);
            }
        }
        None
    }
}

impl fmt::Display for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_html())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_html())
    }
}

impl fmt::Display for HtmlFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_html())
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::response::Responder<'r, 'static> for HtmlFragment {
    fn respond_to(
        self,
        req: &'r rocket::request::Request<'_>,
    ) -> rocket::response::Result<'static> {
        let content = self.to_string();
        rocket::response::Response::build_from(content.respond_to(req)?)
            .header(rocket::http::ContentType::HTML)
            .ok()
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for HtmlFragment {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html(self.to_string()).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let mut body = ElementData::new("body".to_string());

        let text = Node::Text("Hello, Rust!".to_string());
        body.add_child(text);

        let mut div = ElementData::new("div".to_string());
        div.set_attribute("class".to_string(), "container".to_string());

        let inner_text = Node::Text("This is inside a div.".to_string());
        div.add_child(inner_text);

        body.add_child(Node::Element(div));

        let document = Node::Element(body);

        assert_eq!(
            document.to_html(),
            "<body>Hello, Rust!<div class=\"container\">This is inside a div.</div></body>"
        )
    }
}
