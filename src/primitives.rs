/*!
This module contains the basic primitives that are used to construct
all the higher-level EBML Elements.
*/

/// Represents a Variable Length Integer, otherwise known as a VINT.
/// It is an integer value that is represented using a variable number of octets.
pub struct VInt {
    pub(crate) value: i64,
}

impl VInt {
    /// Construct a new VINT.
    pub fn new(value: i64) -> VInt {
        VInt { value: value }
    }

    /// Return the integer value represented by a VINT.
    pub fn value(&self) -> i64 {
        self.value
    }
}
