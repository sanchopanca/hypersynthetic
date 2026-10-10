//! Searching the tree, like the DOM's `getElementById()` and friends.
//!
//! Everything goes in tree order: depth-first, an element before its children.

use std::slice;

use crate::{ElementData, HtmlFragment, Node};

/// Iterator returned by [HtmlFragment::descendants] and [ElementData::descendants]:
/// every element below, in tree order.
#[derive(Clone, Debug)]
pub struct Descendants<'a> {
    // The nodes still to visit, one iterator per level of the tree
    stack: Vec<slice::Iter<'a, Node>>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = &'a ElementData;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let level = self.stack.last_mut()?;
            match level.next() {
                Some(Node::Element(element)) => {
                    // Its children come next, before its siblings
                    self.stack.push(element.child_nodes().iter());
                    return Some(element);
                }
                Some(_) => {}
                None => {
                    self.stack.pop();
                }
            }
        }
    }
}

impl HtmlFragment {
    /// Every element in the fragment, at any depth, in tree order: depth-first, an
    /// element before its children.
    pub fn descendants(&self) -> Descendants<'_> {
        Descendants {
            stack: vec![self.child_nodes().iter()],
        }
    }

    /// The first element with this `id`, in tree order. Like the DOM's
    /// `getElementById()`.
    pub fn get_element_by_id(&self, id: &str) -> Option<&ElementData> {
        self.descendants().find(|element| element.id() == Some(id))
    }

    /// The elements with this tag name, compared ignoring ASCII case, or all
    /// elements for `"*"`. Like the DOM's `getElementsByTagName()`, but an iterator
    /// rather than a live list.
    pub fn get_elements_by_tag_name(&self, name: &str) -> impl Iterator<Item = &ElementData> {
        self.descendants()
            .filter(move |element| name == "*" || element.tag_name.eq_ignore_ascii_case(name))
    }

    /// The elements that have all of these classes, given separated by whitespace:
    /// `"card featured"`. Like the DOM's `getElementsByClassName()`, but an iterator
    /// rather than a live list. No classes match no elements.
    pub fn get_elements_by_class_name(&self, names: &str) -> impl Iterator<Item = &ElementData> {
        let wanted: Vec<&str> = names.split_ascii_whitespace().collect();
        self.descendants()
            .filter(move |element| !wanted.is_empty() && has_classes(element, &wanted))
    }

    /// Like [HtmlFragment::get_element_by_id], to change the element.
    ///
    /// Only the fragments on the way to the element are copied if they're shared
    /// with a clone (see [HtmlFragment::child_nodes_mut]).
    pub fn get_element_by_id_mut(&mut self, id: &str) -> Option<&mut ElementData> {
        // Find it first, without changing anything, then walk to it with `&mut`:
        // a search with `&mut` all the way would copy every shared fragment it
        // looks through
        let path = path_to(self.child_nodes(), &|element| element.id() == Some(id))?;
        Some(element_at_mut(self, &path))
    }

    /// Calls `f` with every element in the fragment, at any depth, in tree order.
    /// `f` runs before an element's children are visited, so children it adds are
    /// visited too.
    ///
    /// This is the way to change several elements: an iterator of `&mut` elements
    /// can't exist, since an element and its descendants would be borrowed at once.
    /// To change only some, check inside `f`:
    ///
    /// ```
    /// # use hypersynthetic::prelude::*;
    /// let mut page = html! { <a href="/">"Home"</a><p><a href="/about">"About"</a></p> };
    /// page.for_each_descendant_mut(|element| {
    ///     if element.tag_name == "a" {
    ///         element.class_list_mut().add("link");
    ///     }
    /// });
    /// assert_eq!(page.get_elements_by_class_name("link").count(), 2);
    /// ```
    ///
    /// Every fragment it goes through is copied if it's shared with a clone, since
    /// `f` could change any element.
    pub fn for_each_descendant_mut(&mut self, mut f: impl FnMut(&mut ElementData)) {
        for_each_mut(self, &mut f);
    }
}

/// The same searches below an element, without the element itself, like the DOM.
impl ElementData {
    /// Every element below this one, at any depth, in tree order. See
    /// [HtmlFragment::descendants].
    pub fn descendants(&self) -> Descendants<'_> {
        self.children.descendants()
    }

    /// See [HtmlFragment::get_element_by_id].
    pub fn get_element_by_id(&self, id: &str) -> Option<&ElementData> {
        self.children.get_element_by_id(id)
    }

    /// See [HtmlFragment::get_elements_by_tag_name].
    pub fn get_elements_by_tag_name(&self, name: &str) -> impl Iterator<Item = &ElementData> {
        self.children.get_elements_by_tag_name(name)
    }

    /// See [HtmlFragment::get_elements_by_class_name].
    pub fn get_elements_by_class_name(&self, names: &str) -> impl Iterator<Item = &ElementData> {
        self.children.get_elements_by_class_name(names)
    }

    /// See [HtmlFragment::get_element_by_id_mut].
    pub fn get_element_by_id_mut(&mut self, id: &str) -> Option<&mut ElementData> {
        self.children.get_element_by_id_mut(id)
    }

    /// See [HtmlFragment::for_each_descendant_mut].
    pub fn for_each_descendant_mut(&mut self, f: impl FnMut(&mut ElementData)) {
        self.children.for_each_descendant_mut(f);
    }
}

/// Whether the element has all the `wanted` classes. Reads the attribute directly,
/// without building a [ClassList](crate::ClassList) for every element searched.
fn has_classes(element: &ElementData, wanted: &[&str]) -> bool {
    let classes = element.get_attribute("class").unwrap_or("");
    wanted
        .iter()
        .all(|name| classes.split_ascii_whitespace().any(|class| class == *name))
}

/// The indexes leading to the first element that matches, one per level.
fn path_to(nodes: &[Node], matches: &impl Fn(&ElementData) -> bool) -> Option<Vec<usize>> {
    for (index, node) in nodes.iter().enumerate() {
        if let Node::Element(element) = node {
            if matches(element) {
                return Some(vec![index]);
            }
            if let Some(mut path) = path_to(element.child_nodes(), matches) {
                path.insert(0, index);
                return Some(path);
            }
        }
    }
    None
}

/// The element at the end of a path from [path_to].
fn element_at_mut<'a>(fragment: &'a mut HtmlFragment, path: &[usize]) -> &'a mut ElementData {
    let (&last, parents) = path.split_last().expect("paths aren't empty");
    let mut nodes = fragment.child_nodes_mut();
    for &index in parents {
        nodes = match &mut nodes[index] {
            Node::Element(element) => element.child_nodes_mut(),
            _ => unreachable!("paths only go through elements"),
        };
    }
    match &mut nodes[last] {
        Node::Element(element) => element,
        _ => unreachable!("paths end at an element"),
    }
}

fn for_each_mut<F: FnMut(&mut ElementData)>(fragment: &mut HtmlFragment, f: &mut F) {
    for node in fragment.child_nodes_mut() {
        if let Node::Element(element) = node {
            f(element);
            for_each_mut(&mut element.children, f);
        }
    }
}
