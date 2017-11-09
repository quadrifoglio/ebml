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

/// Contains all the information about an EBML element.
pub struct ElementInfo {
    id: i64,
    size: usize,
}

impl ElementInfo {
    /// Get the EBML Element ID.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Get the size of the data contained withing the element.
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Represents an EBML Element.
/// Every EBML element consists of two things: its information (ID & expected data size), and the
/// data that is contains.
pub struct Element {
    info: ElementInfo,
    data: Vec<u8>,
}

impl Element {
    /// Create a new EBML Element.
    pub fn new(id: i64, data: Vec<u8>) -> Element {
        let info = ElementInfo {
            id: id,
            size: data.len(),
        };

        Element {
            info: info,
            data: data,
        }
    }

    /// Get information about this EBML element, such as its ID and its expected data size.
    pub fn info<'a>(&'a self) -> &'a ElementInfo {
        &self.info
    }

    /// Get a reference to the element's data.
    pub fn data<'a>(&'a self) -> &'a Vec<u8> {
        &self.data
    }

    /// Consume the element and return its raw binary data.
    pub fn data_binary(self) -> Vec<u8> {
        self.data
    }

    /// Consume the element and return its data as a signed integer.
    pub fn data_i64(self) -> i64 {
        let data = self.data();
        let mut value = 0 as i64;

        for i in 0..data.len() {
            value |= (data[data.len() - i - 1] as i64) << i * 8;
        }

        value
    }

    /// Consume the element and return its data as a 32-bits floating point number.
    pub fn data_f32(self) -> f32 {
        f32::from_bits(self.data_i64() as u32)
    }

    /// Consume the element and return its data as a 64-bits floating point number.
    pub fn data_f64(self) -> f64 {
        f64::from_bits(self.data_i64() as u64)
    }

    /// Comsume the element and return its data as a UTF-8 string.
    pub fn data_utf8(self) -> Result<String> {
        Ok(String::from_utf8(self.data)?)
    }
}

#[cfg(test)]
mod tests;
