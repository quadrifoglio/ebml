/*!
This library is a basic implementation of EBML (Extensible Binary Markup Language),
a binary format for storing hierarchical, typed in data in a compact, yet easily
parsed format. It is used in the MKV Video container format.
*/

#[macro_use]
extern crate error_chain;

extern crate byteorder;

pub mod error;
pub mod io;

/// Type alias for the EBML Element ID.
pub type ElementId = i64;

/// Represents all the EBML data types available.
pub enum Data {
    Master(Vec<Element>),
    Binary(Vec<u8>),
    SignedInteger(i64),
    UnsignedInteger(u64),
    Float32(f32),
    Float64(f32),
    Utf8(String),
    Date(i64),
}

/// Represents an EBML Element.
pub struct Element {
    id: ElementId,
    size: usize,
    data: Data,
}

impl Element {
    /// Create a new EBML Element.
    pub fn new(id: ElementId, size: usize, data: Data) -> Element {
        Element {
            id: id,
            size: size,
            data: data,
        }
    }

    /// Check is this element is a Master element (i.e. contains other child elements).
    pub fn is_master(&self) -> bool {
        match self.data {
            Data::Master(_) => true,
            _ => false,
        }
    }

    /// Return a reference to the element's children, if it has any (i.e. if it is a master
    /// element).
    pub fn children<'a>(&'a self) -> Option<&'a Vec<Element>> {
        if let Data::Master(ref children) = self.data {
            Some(children)
        } else {
            None
        }
    }

    /// Get the EBML Element ID.
    pub fn id(&self) -> ElementId {
        self.id
    }

    /// Get the size of the data contained withing the element.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get a reference to the element's data.
    pub fn data<'a>(&'a self) -> &'a Data {
        &self.data
    }
}

#[cfg(test)]
mod tests;
