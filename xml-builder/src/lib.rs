use std::{fmt, marker::PhantomData};

pub mod attribute;
pub mod document;
pub mod element;

///
/// Типаж, описывающий объект (узел) XML.
///
pub trait XmlObject: fmt::Debug + fmt::Display {
    ///
    /// Получить итератор для последовательного перебора всех атрибутов узла XML.
    ///
    fn attributes(&'static self) -> Box<dyn Iterator<Item = &attribute::XmlAttribute>> {
        Box::new(EmptyAttributesIter::<'_> {
            phantom: PhantomData::<&'_ attribute::XmlAttribute> {},
        })
    }

    ///
    /// Получить итератор для последовательного перебора всех дочерних узлов
    /// данного узла XML.
    ///
    fn children(&'static self) -> Box<dyn Iterator<Item = &dyn XmlObject>> {
        Box::new(EmptyChildrenIter::<'_> {
            phantom: PhantomData::<&'_ dyn XmlObject> {},
        })
    }

    ///
    /// Преобразовать данный узел в формат XML.
    ///
    fn to_xml(&self) -> String;
}

///
/// Пустой итератор для атрибутов узла XML.
///
pub(crate) struct EmptyAttributesIter<'a> {
    phantom: PhantomData<&'a attribute::XmlAttribute>,
}

impl<'a> Iterator for EmptyAttributesIter<'a> {
    type Item = &'a attribute::XmlAttribute;

    ///
    /// Данный итератор сразу завершает перебор,
    /// что говорит о том, что множество атрибутов данного
    /// узла пусто.
    ///
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

///
/// Пустой итератор для дочерних узлов узла XML.
///
pub(crate) struct EmptyChildrenIter<'a> {
    phantom: PhantomData<&'a dyn XmlObject>,
}

impl<'a> Iterator for EmptyChildrenIter<'a> {
    type Item = &'a dyn XmlObject;

    ///
    /// Данный итератор сразу завершает перебор,
    /// что говорит о том, что множество дочерних узлов
    /// данного узла пусто.
    ///
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
