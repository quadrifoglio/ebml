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
pub enum ElementKind {
    Master,
    Binary,
    SignedInteger,
    UnsignedInteger,
    Float,
    Utf8,
    Date,
}

/// Represents an EBML Element.
pub struct Element {
    id: ElementId,
    data: Vec<u8>,
}

impl Element {
    /// Create a new EBML Element.
    pub fn new(id: ElementId, data: Vec<u8>) -> Element {
        Element { id: id, data: data }
    }

    /// Get the EBML Element ID.
    pub fn id(&self) -> ElementId {
        self.id
    }

    /// Get the size of the data contained withing the element.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Get a reference to the element's data.
    pub fn data<'a>(&'a self) -> &'a Vec<u8> {
        &self.data
    }
}

#[cfg(test)]
mod tests;
