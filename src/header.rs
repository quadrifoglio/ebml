//! EBML Header data types.

use element::types::*;

ebml_container_element!(Root => 0x1a45dfa3);

ebml_default_element!(Version => 0x4286, UnsignedInt, 1);
ebml_default_element!(ReadVersion => 0x42f7, UnsignedInt, 1);
ebml_default_element!(MaxIdLength => 0x42f2, UnsignedInt, 4);
ebml_default_element!(MaxSizeLength => 0x42f3, UnsignedInt, 8);

ebml_mandatory_element!(DocType => 0x4282, Utf8);
ebml_mandatory_element!(DocTypeVersion => 0x4287, UnsignedInt);
ebml_mandatory_element!(DocTypeReadVersion => 0x4285, UnsignedInt);
