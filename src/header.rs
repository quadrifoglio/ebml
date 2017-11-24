//! EBML Header data types.

use element::types::*;

pub const EBML: UnsignedInt = 0x1a45dfa3;
pub const VERSION: UnsignedInt = 0x4286;
pub const READ_VERSION: UnsignedInt = 0x42f7;
pub const MAX_ID_LENGTH: UnsignedInt = 0x42f2;
pub const MAX_SIZE_LENGTH: UnsignedInt = 0x42f3;
pub const DOC_TYPE: UnsignedInt = 0x4282;
pub const DOC_TYPE_VERSION: UnsignedInt = 0x4287;
pub const DOC_TYPE_READ_VERSION: UnsignedInt = 0x4285;

ebml_container_element!(Root => EBML);

ebml_default_element!(Version => VERSION, UnsignedInt, 1);
ebml_default_element!(ReadVersion => READ_VERSION, UnsignedInt, 1);
ebml_default_element!(MaxIdLength => MAX_ID_LENGTH, UnsignedInt, 4);
ebml_default_element!(MaxSizeLength => MAX_SIZE_LENGTH, UnsignedInt, 8);

ebml_mandatory_element!(DocType => DOC_TYPE, Utf8);
ebml_mandatory_element!(DocTypeVersion => DOC_TYPE_VERSION, UnsignedInt);
ebml_mandatory_element!(DocTypeReadVersion => DOC_TYPE_READ_VERSION, UnsignedInt);
