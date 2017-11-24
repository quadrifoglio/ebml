//! EBML Element types & helper macros.

use error::{Result, ErrorKind};

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

/// Represents data contained within an EBML element.
pub enum Data {
    Ignored,
    Buffer(Vec<u8>),
}

impl Data {
    /// Return the element data as a raw binary buffer.
    pub fn into_binary(self) -> Result<types::Binary> {
        if let Data::Buffer(buf) = self {
            return Ok(buf);
        }

        Err(ErrorKind::InvalidDataType.into())
    }

    /// Interpret the element data as an unsigned integer.
    pub fn into_unsigned_int(self) -> Result<types::UnsignedInt> {
        if let Data::Buffer(buf) = self {
            let mut value = 0 as u64;

            for i in 0..buf.len() {
                value |= (buf[buf.len() - i - 1] as u64) << i * 8;
            }

            return Ok(value);
        }

        Err(ErrorKind::InvalidDataType.into())
    }

    /// Interpret the element data as a signed integer.
    pub fn into_signed_int(self) -> Result<types::SignedInt> {
        if let Data::Buffer(buf) = self {
            let mut value = 0 as i64;

            for i in 0..buf.len() {
                value |= (buf[buf.len() - i - 1] as i64) << i * 8;
            }

            return Ok(value);
        }

        Err(ErrorKind::InvalidDataType.into())
    }

    /// Interpret the element data as a floating point number.
    pub fn into_float(self) -> Result<types::Float> {
        let len: usize;

        if let Data::Buffer(ref buf) = self {
            len = buf.len();
        } else {
            return Err(ErrorKind::InvalidDataType.into());
        }

        if len == 4 {
            Ok(f32::from_bits(self.into_unsigned_int()? as u32) as types::Float)
        } else if len == 8 {
            Ok(f64::from_bits(self.into_unsigned_int()?))
        } else {
            Err(ErrorKind::InvalidFloatSize.into())
        }
    }

    /// Interpret the element data as a UTF-8 string.
    pub fn into_utf8(self) -> Result<types::Utf8> {
        if let Data::Buffer(buf) = self {
            return Ok(String::from_utf8(buf)?);
        }

        Err(ErrorKind::InvalidDataType.into())
    }
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
