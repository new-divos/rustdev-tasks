use std::{collections::LinkedList, fmt};

use crate::{attribute::XmlAttribute, XmlObject};

///
/// Тэггированный элемент XML.
///
#[derive(Debug)]
pub struct XmlElement {
    // Тэг элемента XML.
    tag: String,

    // Список атрибутов.
    attributes: LinkedList<XmlAttribute>,

    // Список дочерних элементов.
    children: Vec<Box<dyn XmlObject>>,
}

impl fmt::Display for XmlElement {
    ///
    /// Отобразить содержимое элемента XML.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.tag)?;
        if !self.attributes.is_empty() {
            write!(f, " ")?;
            for (idx, attr) in self.attributes.iter().enumerate() {
                if idx > 0 {
                    write!(f, ", ")?;
                }
                attr.fmt(f)?;
            }
        }

        if self.children.is_empty() {
            write!(f, "/>")?;
        } else {
            write!(f, ">")?;
            for e in self.children.iter() {
                e.fmt(f)?;
            }
            write!(f, "</{}>", self.tag)?;
        }

        Ok(())
    }
}

impl XmlObject for XmlElement {
    ///
    /// Получить итератор для перечисления всех атрибутов элеметра XML.
    ///
    fn attributes(&'static self) -> Box<dyn Iterator<Item = &XmlAttribute>> {
        Box::new(self.attributes.iter())
    }

    ///
    /// Получить итератор для перечисления всех дочерних элементов
    /// элемента XML.
    ///
    fn children(&'static self) -> Box<dyn Iterator<Item = &dyn XmlObject>> {
        Box::new(self.children.iter().map(|r| r.as_ref()))
    }

    ///
    /// Преобразовать элемент XML в формат XML.
    ///
    fn to_xml(&self) -> String {
        let mut element = vec![format!("<{}", self.tag)];

        if !self.attributes.is_empty() {
            element.push(" ".to_string());
            element.push(
                self.attributes
                    .iter()
                    .map(|a| a.to_xml())
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }

        if self.children.is_empty() {
            element.push("/>".to_string());
        } else {
            element.push(">".to_string());
            element.extend(self.children.iter().map(|e| e.to_xml()));
            element.push(format!("</{}>", self.tag));
        }

        element.join("")
    }
}

impl XmlElement {
    ///
    /// Создать элемент XML с заданным тэгом.
    ///
    #[inline]
    pub fn new<T: AsRef<str>>(tag: T) -> Self {
        Self {
            tag: tag.as_ref().to_string(),
            attributes: LinkedList::new(),
            children: Vec::new(),
        }
    }

    ///
    /// Создать элемент XML с заданными тэгом и текстом.
    ///
    #[inline]
    pub fn with_text<T: AsRef<str>, X: AsRef<str>>(tag: T, text: X) -> Self {
        Self {
            tag: tag.as_ref().to_string(),
            attributes: LinkedList::new(),
            children: vec![Box::new(XmlText::new(text))],
        }
    }

    ///
    /// Получить тэг элемента XML.
    ///
    #[inline]
    pub fn tag(&self) -> &str {
        self.tag.as_str()
    }

    ///
    /// Добавить атрибут к элементу XML.
    ///
    pub fn add_attribute(&mut self, attr: XmlAttribute) -> &mut Self {
        self.attributes.push_back(attr);

        self
    }

    ///
    /// Добавить дочерний элемент к элементу XML.
    ///
    pub fn add_child<E: 'static + XmlObject>(&mut self, child: E) -> &mut Self {
        self.children.push(Box::new(child));

        self
    }

    ///
    /// Добавить текст к элементу XML.
    ///
    pub fn add_text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.children.push(Box::new(XmlText::new(text)));

        self
    }

    ///
    /// Добавить комментарий к элементу XML.
    ///
    pub fn add_comment<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.children.push(Box::new(XmlComment::new(text)));

        self
    }
}

///
/// Текстовый элемент XML.
///
#[derive(Debug, Clone)]
pub struct XmlText {
    text: String,
}

impl fmt::Display for XmlText {
    ///
    /// Отобразить текстовый объект XML.
    ///
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl XmlObject for XmlText {
    ///
    /// Преобразовать текстовый элемент XML в формат XML.
    ///
    #[inline]
    fn to_xml(&self) -> String {
        self.text.clone()
    }
}

impl XmlText {
    ///
    /// Создать текстовый элемент XML с заданным содержимым.
    ///
    #[inline]
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
        }
    }

    ///
    /// Получить текст текстового элемента XML.
    ///
    #[inline]
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}

///
/// Элемент комментария XML.
///
#[derive(Debug, Clone)]
pub struct XmlComment {
    text: String,
}

impl fmt::Display for XmlComment {
    ///
    /// Отобразить элемент комментария XML.
    ///
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<!--{}-->", self.text)
    }
}

impl XmlObject for XmlComment {
    ///
    /// Преобразовать элемент комментария XML в формат XML.
    ///
    #[inline]
    fn to_xml(&self) -> String {
        format!("<!--{}-->", self.text)
    }
}

impl XmlComment {
    ///
    /// Создать элемент комментария XML с заданным содержимым.
    ///
    #[inline]
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
        }
    }

    ///
    /// Получить текст элемента комментария XML.
    ///
    #[inline]
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}
