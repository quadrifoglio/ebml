use std::io::Cursor;

use header;
use reader::Reader;

#[test]
fn ebml_header_sequential() {
    let data = Cursor::new(vec![
        0x1a, 0x45, 0xdf, 0xa3, 0x93, 0x42, 0x82, 0x88, 0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b,
        0x61, 0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01,
    ]);

    let mut r = Reader::from(data);

    let (root, _) = r.read_element().unwrap();
    let mut count = 0 as usize;

    while count < root.size() {
        let (child, c) = r.read_element().unwrap();
        count += c;

        let data = r.read_element_data(child.size()).unwrap();
        count += child.size();

        match child.id() {
            header::VERSION => assert_eq!(data.to_unsigned_int().unwrap(), 1),
            header::READ_VERSION => assert_eq!(data.to_unsigned_int().unwrap(), 1),
            header::MAX_ID_LENGTH => assert_eq!(data.to_unsigned_int().unwrap(), 4),
            header::MAX_SIZE_LENGTH => assert_eq!(data.to_unsigned_int().unwrap(), 8),

            header::DOC_TYPE => assert_eq!(data.to_utf8().unwrap().as_str(), "matroska"),
            header::DOC_TYPE_VERSION => assert_eq!(data.to_unsigned_int().unwrap(), 1),
            header::DOC_TYPE_READ_VERSION => assert_eq!(data.to_unsigned_int().unwrap(), 1),

            _ => panic!("Unexpected EBML element: 0x{:X}", child.id()),
        };
    }
}
