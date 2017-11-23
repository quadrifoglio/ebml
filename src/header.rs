//! EBML Header data types.

use types;

ebml_container_element!(Header => 0x1a45dfa3, {
    version: Version,
    doc_type: DocType
});

ebml_simple_element!(Version => 0x4286, types::UnsignedInt);
ebml_simple_element!(ReadVersion => 0x42f7, types::UnsignedInt);
ebml_simple_element!(MaxIdLength => 0x42f2, types::UnsignedInt);
ebml_simple_element!(MaxSizeLength => 0x42f3, types::UnsignedInt);
ebml_buffer_element!(DocType => 0x4282, types::Utf8);
ebml_simple_element!(DocTypeVersion => 0x4287, types::UnsignedInt);
ebml_simple_element!(DocTypeReadVersion => 0x4285, types::UnsignedInt);
