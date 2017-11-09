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

/// Represents an EBML Element.
pub struct Element {
    info: ElementInfo,
    data: Vec<u8>,
}

impl Element {
    /// Create a new EBML Element.
    pub fn new(mut info: ElementInfo, data: Vec<u8>) -> Element {
        info.size = data.len();

        Element {
            info: info,
            data: data,
        }
    }

    /// Get the EBML Element ID.
    pub fn id(&self) -> i64 {
        self.info.id
    }

    /// Get the size of the data contained withing the element.
    pub fn size(&self) -> usize {
        self.info.size
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

    /// Consume the element and return its data as a 64-bits floating point number.
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
