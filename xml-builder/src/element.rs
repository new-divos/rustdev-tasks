use std::{collections::LinkedList, fmt};

use crate::{attribute::XmlAttribute, XmlObject};

pub struct XmlElement {
    name: String,
    attributes: LinkedList<XmlAttribute>,
    children: Vec<Box<dyn XmlObject>>,
}

impl XmlObject for XmlElement {
    fn attributes(&'static self) -> Box<dyn Iterator<Item = &XmlAttribute>> {
        Box::new(self.attributes.iter())
    }

    fn children(&'static self) -> Box<dyn Iterator<Item = &dyn XmlObject>> {
        Box::new(self.children.iter().map(|r| r.as_ref()))
    }

    fn to_xml(&self) -> String {
        todo!()
    }
}

impl XmlElement {
    #[inline]
    pub fn new<N: AsRef<str>>(name: N) -> Self {
        Self {
            name: name.as_ref().to_string(),
            attributes: LinkedList::new(),
            children: Vec::new(),
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct XmlText {
    text: String,
}

impl fmt::Display for XmlText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl XmlObject for XmlText {
    fn to_xml(&self) -> String {
        self.text.clone()
    }
}

impl XmlText {
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
        }
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}
