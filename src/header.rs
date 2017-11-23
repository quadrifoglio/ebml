//! EBML Header data types.

use element;

ebml_container_element!(Root => 0x1a45dfa3, {
    version: Version,
    read_version: ReadVersion,
    max_id_length: MaxIdLength,
    max_size_length: MaxSizeLength,
    doc_type: DocType,
    doc_type_version: DocTypeVersion,
    doc_type_read_version: DocTypeReadVersion
});

ebml_simple_element!(Version => 0x4286, element::types::UnsignedInt);
ebml_simple_element!(ReadVersion => 0x42f7, element::types::UnsignedInt);
ebml_simple_element!(MaxIdLength => 0x42f2, element::types::UnsignedInt);
ebml_simple_element!(MaxSizeLength => 0x42f3, element::types::UnsignedInt);
ebml_simple_element!(DocType => 0x4282, element::types::Utf8);
ebml_simple_element!(DocTypeVersion => 0x4287, element::types::UnsignedInt);
ebml_simple_element!(DocTypeReadVersion => 0x4285, element::types::UnsignedInt);
