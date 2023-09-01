pub use htmlize::{escape_attribute, escape_text};

/// The component macro provides a way to define reusable and self-contained web components.
/// A component is a function that returns a [NodeCollection]. The easiest way to create one is
/// [html] macro.
///
/// # Basic Usage
///
/// ```
/// # use hypersynthetic::prelude::*;
/// #[component]
/// fn MyComponent(prop1: &str, prop2: i32) -> NodeCollection {
///    html! {
///        <div>
///            <p>{prop1}</p>
///            <p>{prop2}</p>
///        </div>
///    }
/// }
///
/// # fn main() {
///     let html = html! {
///         <MyComponent prop1="Hello" prop2={42} />
///     };
/// # }
/// ```
///
/// A component name must start with an uppercase letter and it must return a [NodeCollection].
///
/// # Props
/// Components accept properties, similar to function arguments.
/// These properties can be any Rust type, providing a type-safe way to pass data to components.
/// When calling a component from [html] macro, the properties are passed as html arguments.
/// Currently, they need to be passed in the same order as they are defined in the component.
/// See [github issue](https://github.com/sanchopanca/hypersynthetic/issues/8) for more details.
///
/// # Slots
/// [TODO](https://github.com/sanchopanca/hypersynthetic/issues/10)
pub use hypersynthetic_macros::component;

/// The `html` macro allows to construct html fragments in Rust.
/// It mostly follows the same syntax as HTML with the following exceptions:
///
/// 1. Void tags need to be self-closing. For example `<br>` should be written as `<br />`.
///
/// 2. Bare text is not (yet) allowed. Instead you should put a string literal inside an expression
/// like `<span>{"text"}</span>`. Alternatively, for convenience, you can use the string literal
/// directly like `<span>"text"</span>`. But the latter syntax is likely to change in the future.
/// See this [github issue](https://github.com/sanchopanca/hypersynthetic/issues/3) for details.
///
/// With gotchas out the way, here are the feautures:
///
/// # Dynamic content
/// An expression that implements Display trait (or just ToString trait) inside curly braces will be substituted
/// with the resulst of .to_string() call, applying html escaping (opting out from escaping is not yet possible,
/// see this [github issue](https://github.com/sanchopanca/hypersynthetic/issues/5).
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
/// assert_eq!(div.to_string(), "<div class=\"header\">Breaking news</div>");
///
/// let id = 42;
/// let span = html! {
///     <span id="header-{id}">{"Breaking news"}</span>
/// };
///
/// assert_eq!(span.to_string(), "<span id=\"header-42\">Breaking news</span>");
/// ```
///
/// 3. In an attribute name.
/// ```
/// # use hypersynthetic::html;
/// let method = "hx-get";
/// let div = html! {
///     <button hx-get="/resources">{"Get 'em"}</button>
/// };
/// assert_eq!(div.to_string(), "<button hx-get=\"/resources\">Get 'em</button>");
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
/// assert_eq!(div.to_string(), "<input type=\"text\" value=\"1\" /><input type=\"text\" value=\"2\" />");
/// ```
///
/// # Components
/// Components can be called as self-closing tags. Here is an example:
/// ```
/// # use hypersynthetic::prelude::*;
/// #[component]
/// fn MyDiv(text: &str) -> NodeCollection {
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
    pub use crate::NodeCollection;
    pub use crate::{component, html};
}

use std::fmt;

#[derive(Clone, Debug)]
pub enum NodeCollection {
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
    pub children: NodeCollection,
    pub self_closing: bool,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl NodeCollection {
    // TODO: consider something else except Vec
    pub fn new(nodes: Vec<Node>) -> Self {
        NodeCollection::Nodes(nodes)
    }

    pub fn push(&mut self, node: Node) {
        match self {
            NodeCollection::Nodes(nodes) => nodes.push(node),
        }
    }

    fn to_html(&self) -> String {
        match self {
            NodeCollection::Nodes(nodes) => nodes.iter().map(|node| node.to_html()).collect(),
        }
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        match self {
            NodeCollection::Nodes(nodes) => nodes.clone(),
        }
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
            children: NodeCollection::new(Vec::new()),
            self_closing: false,
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.push(Attribute {
            name,
            value: Some(value),
        });
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
            NodeCollection::Nodes(nodes) => nodes.iter().map(|node| node.to_html()).collect(),
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

impl fmt::Display for NodeCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_html())
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
        div.add_attribute("class".to_string(), "container".to_string());

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
