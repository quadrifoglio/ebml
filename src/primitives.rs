/*!
This module contains the basic primitives that are used to construct
all the higher-level EBML Elements.
*/

/// Represents a Variable Length Integer, otherwise known as a VINT.
/// It is an integer value that is represented using a variable number of octets.
pub struct VInt {
    pub(crate) length: usize,  // The length of the integer in octets.
    pub(crate) value: Vec<u8>, // The value of the VINT, `length` octets long.
}
