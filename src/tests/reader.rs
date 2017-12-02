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

    let (elem, _) = r.read_element(false).unwrap();
    assert_eq!(elem.id(), header::EBML);

    let (elem, _) = r.read_element(true).unwrap();
    assert_eq!(elem.id(), header::DOC_TYPE);
    assert_eq!(elem.data().to_utf8().unwrap().as_str(), "matroska");

    let (elem, _) = r.read_element(true).unwrap();
    assert_eq!(elem.id(), header::DOC_TYPE_VERSION);
    assert_eq!(elem.data().to_unsigned_int().unwrap(), 1 as u64);

    let (elem, _) = r.read_element(true).unwrap();
    assert_eq!(elem.id(), header::DOC_TYPE_READ_VERSION);
    assert_eq!(elem.data().to_unsigned_int().unwrap(), 1 as u64);
}

#[test]
fn ebml_header_children() {
    let data = Cursor::new(vec![
        0x1a, 0x45, 0xdf, 0xa3, 0x93, 0x42, 0x82, 0x88, 0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b,
        0x61, 0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01,
    ]);

    let mut r = Reader::from(data);

    let (header, _) = r.read_element(true).unwrap();
    assert_eq!(header.id(), header::EBML);

    let dt = header.find(header::DOC_TYPE).unwrap();
    assert_eq!(dt.data().to_utf8().unwrap().as_str(), "matroska");

    let dt_version = header.find(header::DOC_TYPE_VERSION).unwrap();
    assert_eq!(dt_version.data().to_unsigned_int().unwrap(), 1 as u64);

    let dt_read_version = header.find(header::DOC_TYPE_READ_VERSION).unwrap();
    assert_eq!(dt_read_version.data().to_unsigned_int().unwrap(), 1 as u64);
}
