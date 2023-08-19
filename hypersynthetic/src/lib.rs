pub use htmlize::{escape_attribute, escape_text};
pub use hypersynthetic_macros::{component, html};

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

    pub fn to_html(&self) -> String {
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
    pub fn to_html(&self) -> String {
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

        // let children_string: String = self.children.iter().map(|child| child.to_html()).collect();

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
