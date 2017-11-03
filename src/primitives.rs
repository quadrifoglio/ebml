/*!
This module contains the basic primitives that are used to construct
all the higher-level EBML Elements.
*/

/// Represents a Variable Length Integer, otherwise known as a VINT.
/// It is an integer value that is represented using a variable number of octets.
pub struct VInt {
    pub(crate) length: usize, // The length of the integer in octets.
    pub(crate) value: i64,    // The value of the VINT, muse be representable in `length` octets.
}

impl VInt {
    /// Return the length in bytes of this VINT.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Return the integer value represented by a VINT.
    pub fn value(&self) -> i64 {
        self.value
    }
}
