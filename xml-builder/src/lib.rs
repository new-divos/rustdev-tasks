use std::marker::PhantomData;

pub mod attribute;
pub mod element;

pub trait XmlObject {
    fn attributes(&'static self) -> Box<dyn Iterator<Item = &attribute::XmlAttribute>> {
        Box::new(EmptyAttributesIter::<'_> {
            phantom: PhantomData::<&'_ attribute::XmlAttribute> {},
        })
    }

    fn children(&'static self) -> Box<dyn Iterator<Item = &dyn XmlObject>> {
        Box::new(EmptyChildrenIter::<'_> {
            phantom: PhantomData::<&'_ dyn XmlObject> {},
        })
    }

    fn to_xml(&self) -> String;
}

pub(crate) struct EmptyAttributesIter<'a> {
    phantom: PhantomData<&'a attribute::XmlAttribute>,
}

impl<'a> Iterator for EmptyAttributesIter<'a> {
    type Item = &'a attribute::XmlAttribute;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub(crate) struct EmptyChildrenIter<'a> {
    phantom: PhantomData<&'a dyn XmlObject>,
}

impl<'a> Iterator for EmptyChildrenIter<'a> {
    type Item = &'a dyn XmlObject;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
