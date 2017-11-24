//! EBML Element types & helper macros.

use error::{ErrorKind, Result};

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

    /// Return wether this EBML element is a Master Element, i.e if it contains other EBML elements.
    fn is_master() -> bool;
}

/// Represents data contained within an EBML element.
#[derive(Clone)]
pub struct Data(pub(crate) Option<Vec<u8>>);

impl Data {
    /// Consume the object and returns its data as a binary buffer.
    pub fn take(self) -> Result<types::Binary> {
        if let Some(buf) = self.0 {
            return Ok(buf);
        }

        Err(ErrorKind::NoData.into())
    }

    /// Interpret the element data as a UTF-8 string. This function creates a copy of the data in
    /// order to allocate the new UTF-8 string.
    pub fn to_utf8(&self) -> Result<types::Utf8> {
        if let Some(ref buf) = self.0 {
            return Ok(String::from_utf8(buf.clone())?);
        }

        Err(ErrorKind::NoData.into())
    }

    /// Interpret the element data as an unsigned integer.
    pub fn to_unsigned_int(&self) -> Result<types::UnsignedInt> {
        if let Some(ref buf) = self.0 {
            let mut value = 0 as u64;

            for i in 0..buf.len() {
                value |= (buf[buf.len() - i - 1] as u64) << i * 8;
            }

            return Ok(value);
        }

        Err(ErrorKind::NoData.into())
    }

    /// Interpret the element data as a signed integer.
    pub fn to_signed_int(&self) -> Result<types::SignedInt> {
        if let Some(ref buf) = self.0 {
            let mut value = 0 as i64;

            for i in 0..buf.len() {
                value |= (buf[buf.len() - i - 1] as i64) << i * 8;
            }

            return Ok(value);
        }

        Err(ErrorKind::NoData.into())
    }

    /// Interpret the element data as a floating point number.
    pub fn to_float(&self) -> Result<types::Float> {
        let len: usize;

        if let Some(ref buf) = self.0 {
            len = buf.len();
        } else {
            return Err(ErrorKind::NoData.into());
        }

        if len == 4 {
            Ok(f32::from_bits(self.to_unsigned_int()? as u32)
                as types::Float)
        } else if len == 8 {
            Ok(f64::from_bits(self.to_unsigned_int()?))
        } else {
            Err(ErrorKind::InvalidFloatSize.into())
        }
    }
}

macro_rules! ebml_mandatory_element {
    ($name:ident => $id:expr, $dt:ty) => {
        #[derive(Default)]
        pub struct $name($dt);

        impl ::element::Element for $name {
            fn id() -> ::element::Id {
                $id
            }

            fn is_master() -> bool {
                false
            }
        }
    }
}

macro_rules! ebml_default_element {
    ($name:ident => $id:expr, $dt: ty, $def:expr) => {
        pub struct $name($dt);

        impl ::element::Element for $name {
            fn id() -> ::element::Id {
                $id
            }

            fn is_master() -> bool {
                false
            }
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                $name($def)
            }
        }
    }
}

macro_rules! ebml_container_element {
    ($name:ident => $id:expr) => {
        #[derive(Default)]
        pub struct $name;

        impl ::element::Element for $name {
            fn id() -> ::element::Id {
                $id
            }

            fn is_master() -> bool {
                true
            }
        }
    }
}
