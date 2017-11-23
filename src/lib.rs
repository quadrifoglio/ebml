//! ebml-rs
//!
//! ## Description
//!
//! This library aims to provide a simple and ergonomic way to read and write EBML documents.

#[macro_use]
extern crate error_chain;

/// Type alias for the IDs of EBML elements.
pub type ElementId = u64;

/// Type alias for the size of EBML elements.
pub type ElementSize = usize;

/// Trait that must be implemented by all types that represent an EBML element.
pub trait Element: Default {
    /// Returns the ID of the EBML element.
    fn id() -> ElementId;

    /// Return wether this EBML element has children, i.e if it contains other EBML elements.
    fn has_children() -> bool;
}

macro_rules! ebml_simple_element {
    ($name:ident => $id:expr, $dt:ty) => {
        #[derive(Default)]
        pub struct $name($dt);

        impl ::Element for $name {
            fn id() -> ::ElementId {
                $id
            }

            fn has_children() -> bool {
                false
            }
        }
    }
}

macro_rules! ebml_container_element {
    ($name:ident => $id:expr, { $($member:ident : $type:ty ),* } ) => {
        #[derive(Default)]
        pub struct $name {
            $(
                pub $member: $type,
            )*
        }

        impl ::Element for $name {
            fn id() -> ::ElementId {
                $id
            }

            fn has_children() -> bool {
                true
            }
        }
    }
}

pub mod error;
pub mod types;
pub mod header;
pub mod reader;

#[cfg(test)]
mod tests;
