pub enum Node {
    Element(ElementData),
    Text(String),
}

pub struct ElementData {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Node {
    pub fn to_html(&self) -> String {
        match self {
            Node::Text(text) => text.clone(),
            Node::Element(element_data) => element_data.to_html(),
        }
    }
}

impl ElementData {
    pub fn new(tag_name: String) -> Self {
        ElementData {
            tag_name,
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.push(Attribute { name, value });
    }

    fn to_html(&self) -> String {
        let attributes_string: String = self
            .attributes
            .iter()
            .map(|attr| format!(" {}=\"{}\"", attr.name, attr.value))
            .collect();

        let children_string: String = self.children.iter().map(|child| child.to_html()).collect();

        format!(
            "<{}{}>{}</{}>",
            self.tag_name, attributes_string, children_string, self.tag_name
        )
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
