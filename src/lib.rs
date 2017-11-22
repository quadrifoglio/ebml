//! ebml-rs
//!
//! ## Description
//!
//! This library aims to provide a simple and ergonomic way to read and write EBML documents.

/// Type alias for the IDs of EBML elements.
pub type ElementId = u64;

/// Type alias for the size of EBML elements.
pub type ElementSize = usize;

/// Trait that must be implemented by all types that represent an EBML element.
pub trait Element {
    /// Returns the ID of the EBML element.
    fn id(&self) -> ElementId;

    /// Returns the size in bytes of the data contained in the EBML element.
    fn size(&self) -> ElementSize;

    /// Return wether this EBML element has children, i.e if it contains other EBML elements.
    fn has_children(&self) -> bool;
}

macro_rules! ebml_simple_element {
    ($name:ident => $id:expr, $dt:ty) => {
        pub struct $name($dt);

        impl ::Element for $name {
            fn id(&self) -> ::ElementId {
                $id
            }

            fn size(&self) -> ::ElementSize {
                ::std::mem::size_of::<$dt>()
            }

            fn has_children(&self) -> bool {
                false
            }
        }
    }
}

pub mod types;
pub mod header;
pub mod reader;

#[cfg(test)]
mod tests;
