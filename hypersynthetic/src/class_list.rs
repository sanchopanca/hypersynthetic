//! The DOM's `classList`: an element's classes as a set of names.

use crate::ElementData;

/// The classes of an element, from [ElementData::class_list]: the DOM's `classList`,
/// for reading. To change them, use [ElementData::class_list_mut].
///
/// Like in the DOM, the `class` attribute is read as a set: names are separated by
/// whitespace, and a name that's there twice counts once.
#[derive(Clone, Debug)]
pub struct ClassList<'a> {
    names: Vec<&'a str>,
}

impl<'a> ClassList<'a> {
    pub(crate) fn new(element: &'a ElementData) -> Self {
        ClassList {
            names: parse(element.get_attribute("class").unwrap_or("")),
        }
    }

    /// Whether the element has this class.
    pub fn contains(&self, name: &str) -> bool {
        self.names.contains(&name)
    }

    /// The class names, in order, each once.
    pub fn iter(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.names.iter().copied()
    }

    /// The number of different classes.
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Whether the element has no classes.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

/// The classes of an element, from [ElementData::class_list_mut]: the DOM's
/// `classList`, for changing them.
///
/// Like the DOM, a change writes the `class` attribute back as the names separated by
/// single spaces, without duplicates. Removing the last class leaves `class=""`, and
/// removing from an element without a `class` attribute doesn't add one.
///
/// # Panics
///
/// The methods panic if a class name is empty or contains whitespace, where the DOM
/// throws an exception. Add several classes with several calls:
/// `add("a").add("b")`.
#[derive(Debug)]
pub struct ClassListMut<'a> {
    element: &'a mut ElementData,
}

impl<'a> ClassListMut<'a> {
    pub(crate) fn new(element: &'a mut ElementData) -> Self {
        ClassListMut { element }
    }

    /// Whether the element has this class.
    pub fn contains(&self, name: &str) -> bool {
        self.element.class_list().contains(name)
    }

    /// Adds the class if the element doesn't have it. Returns `self`, so calls can be
    /// chained: `add("a").add("b")`.
    pub fn add(&mut self, name: &str) -> &mut Self {
        check(name);
        let mut names = self.names();
        if !names.iter().any(|existing| existing == name) {
            names.push(name.to_owned());
        }
        self.update(names);
        self
    }

    /// Removes the class. Returns `self`, so calls can be chained.
    pub fn remove(&mut self, name: &str) -> &mut Self {
        check(name);
        let mut names = self.names();
        names.retain(|existing| existing != name);
        self.update(names);
        self
    }

    /// Adds the class if it's missing and removes it if it's there; with `force`,
    /// only adds (`Some(true)`) or only removes (`Some(false)`). Returns whether the
    /// element has the class afterwards.
    pub fn toggle(&mut self, name: &str, force: Option<bool>) -> bool {
        check(name);
        if self.contains(name) {
            if force != Some(true) {
                self.remove(name);
                return false;
            }
            true
        } else {
            if force != Some(false) {
                self.add(name);
                return true;
            }
            false
        }
    }

    /// Replaces the class `old` with `new`, in the same position. Returns whether the
    /// element had `old`; if it didn't, nothing changes.
    pub fn replace(&mut self, old: &str, new: &str) -> bool {
        check(old);
        check(new);
        if !self.contains(old) {
            return false;
        }
        // The first of `old` and `new` becomes `new`, other copies of either go
        let mut replaced = false;
        let names = self
            .names()
            .into_iter()
            .filter_map(|name| {
                if name != old && name != new {
                    Some(name)
                } else if replaced {
                    None
                } else {
                    replaced = true;
                    Some(new.to_owned())
                }
            })
            .collect();
        self.update(names);
        true
    }

    fn names(&self) -> Vec<String> {
        self.element
            .class_list()
            .iter()
            .map(str::to_owned)
            .collect()
    }

    /// The DOM's "update steps": write the names back, unless there's nothing to
    /// write and no attribute to write it to.
    fn update(&mut self, names: Vec<String>) {
        if names.is_empty() && !self.element.has_attribute("class") {
            return;
        }
        self.element.set_attribute("class", names.join(" "));
    }
}

/// Splits on ASCII whitespace, like the DOM, keeping the first of duplicates.
fn parse(value: &str) -> Vec<&str> {
    let mut names: Vec<&str> = Vec::new();
    for name in value.split_ascii_whitespace() {
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

fn check(name: &str) {
    assert!(!name.is_empty(), "class names can't be empty");
    assert!(
        !name.contains(|c: char| c.is_ascii_whitespace()),
        "class name {name:?} contains whitespace: add each class separately"
    );
}
