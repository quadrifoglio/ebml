//! EBML Header data types.

use element::types::UnsignedInt;

pub const EBML: UnsignedInt = 0x1a45dfa3;
pub const VERSION: UnsignedInt = 0x4286;
pub const READ_VERSION: UnsignedInt = 0x42f7;
pub const MAX_ID_LENGTH: UnsignedInt = 0x42f2;
pub const MAX_SIZE_LENGTH: UnsignedInt = 0x42f3;
pub const DOC_TYPE: UnsignedInt = 0x4282;
pub const DOC_TYPE_VERSION: UnsignedInt = 0x4287;
pub const DOC_TYPE_READ_VERSION: UnsignedInt = 0x4285;
