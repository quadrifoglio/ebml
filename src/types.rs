//! EBML data type definitions.

pub type Binary = Vec<u8>;
pub type UnsignedInt = u64;
pub type SignedInt = i64;
pub type Float = f64;
pub type Utf8 = String;

pub type ElementId = UnsignedInt;
pub type ElementSize = usize;
