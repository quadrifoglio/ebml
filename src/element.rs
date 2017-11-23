//! EBML Element types & helper macros.

/// Type alias for the IDs of EBML elements.
pub type Id = u64;

/// Type alias for the size of EBML elements.
pub type Size = usize;

/// Supported EBML elements data types.
pub mod types {
    pub type Binary = Vec<u8>;
    pub type UnsignedInt = u64;
    pub type SignedInt = i64;
    pub type Float = f64;
    pub type Utf8 = String;
}

/// Trait that must be implemented by all types that represent an EBML element.
pub trait Element: Default {
    /// Returns the ID of the EBML element.
    fn id() -> Id;

    /// Return wether this EBML element has children, i.e if it contains other EBML elements.
    fn has_children() -> bool;
}

/// Data contained within an EBML element.
pub enum Data {
    Binary(types::Binary),
    UnsignedInt(types::UnsignedInt),
    SignedInt(types::SignedInt),
    Float(types::Float),
    Utf8(types::Utf8),
}

macro_rules! ebml_simple_element {
    ($name:ident => $id:expr, $dt:ty) => {
        #[derive(Default)]
        pub struct $name($dt);

        impl ::element::Element for $name {
            fn id() -> ::element::Id {
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

        impl ::element::Element for $name {
            fn id() -> ::element::Id {
                $id
            }

            fn has_children() -> bool {
                true
            }
        }
    }
}
