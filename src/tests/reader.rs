//! Reader unit tests.

use std::io::Cursor;

use reader;
use header;

#[test]
fn ebml_header_oneshot() {
    let mut data = Cursor::new(vec![
       0x1a, 0x45, 0xdf, 0xa3, 0x93, 0x42, 0x82, 0x88, 0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b,
       0x61, 0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01,
    ]);

    let (root, _) = reader::read_element(&mut data).unwrap();

    for child in root.content().children().unwrap() {
        let id = child.id();
        let data = child.content();

        match id {
            header::VERSION => assert_eq!(data.into_uint(), 1),
            header::READ_VERSION => assert_eq!(data.into_uint(), 1),
            header::MAX_ID_LENGTH => assert_eq!(data.into_uint(), 4),
            header::MAX_SIZE_LENGTH => assert_eq!(data.into_uint(), 8),

            header::DOC_TYPE => assert_eq!(data.into_utf8().unwrap().as_str(), "matroska"),
            header::DOC_TYPE_VERSION => assert_eq!(data.into_uint(), 1),
            header::DOC_TYPE_READ_VERSION => assert_eq!(data.into_uint(), 1),

            _ => panic!("Unexpected EBML element"),
        };
    }
}
