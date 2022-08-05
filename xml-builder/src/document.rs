use std::fmt;

use crate::{element::XmlComment, XmlObject};

///
/// Документ XML.
///
#[derive(Debug)]
pub struct XmlDocument {
    // Список дочерних элементов.
    children: Vec<Box<dyn XmlObject>>,
}

impl Default for XmlDocument {
    ///
    /// Создать пустой XML документ.
    ///
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for XmlDocument {
    ///
    /// Отобразить содержимое XML документа.
    /// 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        for e in self.children.iter() {
            e.fmt(f)?;
        }

        Ok(())
    }
}

impl XmlObject for XmlDocument {
    ///
    /// Получить итератор для перечисления всех дочерних элементов
    /// документа XML.
    ///
    fn children(&'static self) -> Box<dyn Iterator<Item = &dyn XmlObject>> {
        Box::new(self.children.iter().map(|r| r.as_ref()))
    }

    ///
    /// Преобразовать документ XML в формат XML.
    ///
    fn to_xml(&self) -> String {
        let mut document = vec!["<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string()];
        for e in self.children.iter() {
            document.push(e.to_xml());
        }

        document.join("")
    }
}

impl XmlDocument {
    ///
    /// Создать пустой XML документ.
    ///
    #[inline]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    ///
    /// Добавить дочерний элемент к документу XML.
    ///
    pub fn add_child<E: 'static + XmlObject>(&mut self, child: E) -> &mut Self {
        self.children.push(Box::new(child));

        self
    }

    ///
    /// Добавить комментарий к документу XML.
    ///
    pub fn add_comment<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.children.push(Box::new(XmlComment::new(text)));

        self
    }
}
