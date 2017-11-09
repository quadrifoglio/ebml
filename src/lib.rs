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

use error::Result;

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

    /// Consume the element and return its raw binary data.
    pub fn get_data_binary(self) -> Vec<u8> {
        self.data
    }

    /// Consume the element and return its data as a signed integer.
    pub fn get_data_i64(self) -> i64 {
        let data = self.data();
        let mut value = 0 as i64;

        for i in 0..data.len() {
            value |= (data[data.len() - i - 1] as i64) << i * 8;
        }

        value
    }

    /// Consume the element and return its data as a 32-bits floating point number.
    pub fn get_data_f32(self) -> f32 {
        f32::from_bits(self.get_data_i64() as u32)
    }

    /// Consume the element and return its data as a floating point number.
    pub fn get_data_f64(self) -> f64 {
        f64::from_bits(self.get_data_i64() as u64)
    }

    /// Comsume the element and return its data as a UTF-8 string.
    pub fn get_data_utf8(self) -> Result<String> {
        Ok(String::from_utf8(self.data)?)
    }
}

#[cfg(test)]
mod tests;
