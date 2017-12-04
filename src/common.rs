//! Common data types that are used throughout the library.

use std::io::Cursor;

use reader;
use error::{ErrorKind, Result};

/// All the data types defined by the EBML standard.
pub mod types {
    pub type Binary = Vec<u8>;
    pub type UnsignedInt = u64;
    pub type SignedInt = i64;
    pub type Float = f64;
    pub type Utf8 = String;

    pub type ElementId = UnsignedInt;
    pub type ElementSize = usize;
}

/// Represents an EBML element, with its ID and content.
pub struct Element {
    pub(crate) id: types::ElementId,
    pub(crate) size: types::ElementSize,
    pub(crate) content: ElementContent,
}

impl Element {
    /// Return the ID of the EBML element.
    pub fn id(&self) -> types::ElementId {
        self.id
    }

    /// Return the size of the element's content.
    pub fn size(&self) -> types::ElementSize {
        self.size
    }

    /// Returns the content of the EBML element. Consumes `self`.
    pub fn content(self) -> ElementContent {
        self.content
    }
}

/// EBML element content. Can be either raw user data, or other child EBML elements.
pub struct ElementContent(Vec<u8>);

impl ElementContent {
    /// Create a new Element Content object.
    pub fn new(data: Vec<u8>) -> ElementContent {
        ElementContent(data)
    }

    /// Interpret the element content as raw binary data. Consumes `self`.
    pub fn into_binary(self) -> Vec<u8> {
        self.0
    }

    /// Interpret the element content as an unsigned integer. Consumes `self`.
    pub fn into_uint(self) -> types::UnsignedInt {
        let buf = self.0;
        let mut value = 0 as u64;

        for i in 0..buf.len() {
            value |= (buf[buf.len() - i - 1] as u64) << i * 8;
        }

        value
    }

    /// Interpret the element content as a signed integer. Consumes `self`.
    pub fn into_int(self) -> types::SignedInt {
        let buf = self.0;
        let mut value = 0 as i64;

        for i in 0..buf.len() {
            value |= (buf[buf.len() - i - 1] as i64) << i * 8;
        }

        value
    }

    /// Interpret the element content as a floating point number. Consumes `self`.
    pub fn into_float(self) -> Result<types::Float> {
        if self.0.len() == 4 {
            Ok(f32::from_bits(self.into_uint() as u32) as f64)
        } else if self.0.len() == 8 {
            Ok(f64::from_bits(self.into_uint()))
        } else {
            bail!(ErrorKind::InvalidFloatSize);
        }
    }

    /// Interpret the element content as an UTF-8 string. Can return an error if the data in not
    /// valid UTF-8. Consumes `self`.
    pub fn into_utf8(self) -> Result<types::Utf8> {
        Ok(String::from_utf8(self.into_binary())?)
    }

    /// Interpret the element content as an array of children elements. Consumes `self`.
    pub fn children(self) -> Result<ElementArray> {
        let mut children = Vec::new();

        let len = self.0.len();
        let mut r = Cursor::new(self.0);
        let mut count = 0 as usize;

        while count < len {
            let (elem, c) = reader::read_element(&mut r)?;
            count += c;

            children.push(elem);
        }

        Ok(ElementArray(children))
    }
}

/// Represents a list of child EBML elements.
pub struct ElementArray(Vec<Element>);

impl ElementArray {
    /// Find a specific element based on its ID, and return it. If found, the element is removed
    /// from the list.
    pub fn find(&mut self, id: types::ElementId) -> Option<Element> {
        let mut elem = None;

        for i in 0..self.0.len() {
            if self.0.get(i).unwrap().id() == id {
                elem = Some(self.0.remove(i));
                break;
            }
        }

        elem
    }

    /// Return the list of elements as a Vec. Consumes `self`.
    pub fn vec(self) -> Vec<Element> {
        self.0
    }
}
