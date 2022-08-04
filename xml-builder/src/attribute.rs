use std::fmt;

///
/// Атрибут XML объекта.
///
#[derive(Debug, Clone)]
pub struct XmlAttribute {
    name: String,
    value: String,
}

impl fmt::Display for XmlAttribute {
    ///
    /// Получить строковое представление атрибута.
    ///
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}=\"{}\"", self.name, self.value)
    }
}

impl XmlAttribute {
    ///
    /// Создать атрибут XML с заданными именем и значением.
    ///
    #[inline]
    pub fn new<N: AsRef<str>, V: AsRef<str>>(name: N, value: V) -> Self {
        Self {
            name: name.as_ref().to_string(),
            value: value.as_ref().to_string(),
        }
    }

    ///
    /// Получить имя атрибута.
    ///
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Получить значение атрибута.
    ///
    #[inline]
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    ///
    /// Преобразовать атрибут в формат XML.
    ///
    #[inline]
    pub fn to_xml(&self) -> String {
        format!("{}", *self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes_test() {
        let attr = XmlAttribute::new("key", "value");
        assert_eq!(attr.to_xml(), "key=\"value\"");
        assert_eq!(format!("{attr}"), "key=\"value\"");
    }
}
