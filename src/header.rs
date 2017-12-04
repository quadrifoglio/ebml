//! EBML Header data types.

use common::types::*;

pub const EBML: UnsignedInt = 0x1a45dfa3;
pub const VERSION: UnsignedInt = 0x4286;
pub const READ_VERSION: UnsignedInt = 0x42f7;
pub const MAX_ID_LENGTH: UnsignedInt = 0x42f2;
pub const MAX_SIZE_LENGTH: UnsignedInt = 0x42f3;
pub const DOC_TYPE: UnsignedInt = 0x4282;
pub const DOC_TYPE_VERSION: UnsignedInt = 0x4287;
pub const DOC_TYPE_READ_VERSION: UnsignedInt = 0x4285;

#[derive(Default)]
/// The standard EBML header.
pub struct Header {
    pub(crate) version: UnsignedInt,
    pub(crate) read_version: UnsignedInt,
    pub(crate) max_id_length: UnsignedInt,
    pub(crate) max_size_length: UnsignedInt,
    pub(crate) doc_type: Utf8,
    pub(crate) doc_type_version: UnsignedInt,
    pub(crate) doc_type_read_version: UnsignedInt,
}

impl Header {
    /// Return the version of EBML used to create this file.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Return the minumum version of EBML required to read this file.
    pub fn read_version(&self) -> u64 {
        self.read_version
    }

    /// Return the maximum length in bytes of the EBML element IDs found in this file.
    pub fn max_id_length(&self) -> u64 {
        self.max_id_length
    }

    /// Return the maximum length in bytes of the EBML element sizes found in this file.
    pub fn max_size_length(&self) -> u64 {
        self.max_size_length
    }

    /// Return a string that describes the type of this document.
    pub fn doc_type<'a>(&'a self) -> &'a str {
        self.doc_type.as_str()
    }

    /// Return the version of DocType interpreter used to create this file.
    pub fn doc_type_version(&self) -> u64 {
        self.doc_type_version
    }

    /// Return the minumum version of DocType interpreter required to read this file.
    pub fn doc_type_read_version(&self) -> u64 {
        self.doc_type_read_version
    }
}
