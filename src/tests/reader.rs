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

    for child in root.content().children().unwrap().vec() {
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

#[test]
fn ebml_header_sequential() {
    let mut data = Cursor::new(vec![
       0x1a, 0x45, 0xdf, 0xa3, 0x93, 0x42, 0x82, 0x88, 0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b,
       0x61, 0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01,
    ]);

    let (id, size, _) = reader::read_element_info(&mut data).unwrap();
    let mut count = 0 as usize;

    assert_eq!(id, header::EBML);

    while count < size {
        let (child, c) = reader::read_element(&mut data).unwrap();
        count += c;

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

#[test]
fn data_binary() {
    let mut data = Cursor::new(vec![
        0x8f, 0x8e, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_binary();

    assert_eq!(value, vec![0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
}

#[test]
fn data_unsigned_int_8() {
    let mut data = Cursor::new(vec![
        0x8f, 0x81, 0x2a
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_uint();

    assert_eq!(value, 42);
}

#[test]
fn data_unsigned_int_16() {
    let mut data = Cursor::new(vec![
        0x8f, 0x82, 0x15, 0x6f
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_uint();

    assert_eq!(value, 5487);
}

#[test]
fn data_unsigned_int_24() {
    let mut data = Cursor::new(vec![
        0x8f, 0x83, 0x0e, 0xfd, 0xa1
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_uint();

    assert_eq!(value, 982433);
}

#[test]
fn data_unsigned_int_32() {
    let mut data = Cursor::new(vec![
        0x8f, 0x84, 0x3, 0x3a, 0x3d, 0xdc
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_uint();

    assert_eq!(value, 54148572);
}

#[test]
fn data_unsigned_int_40() {
    let mut data = Cursor::new(vec![
        0x8f, 0x85, 0x07, 0xae, 0x07, 0x7f, 0xcf
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_uint();

    assert_eq!(value, 32984498127);
}

#[test]
fn data_unsigned_int_64() {
    let mut data = Cursor::new(vec![
        0x8f, 0x88, 0x2d, 0xc6, 0x72, 0xa0, 0xf4, 0xe6, 0xea, 0x9a
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_uint();

    assert_eq!(value, 3298449812724574874);
}

#[test]
fn data_signed_int_8() {
    let mut data = Cursor::new(vec![
        0x8f, 0x88, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xD6
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_int();

    assert_eq!(value, -42);
}

#[test]
fn data_signed_int_16() {
    let mut data = Cursor::new(vec![
        0x8f, 0x88, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xea, 0x91
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_int();

    assert_eq!(value, -5487);
}

#[test]
fn data_signed_int_24() {
    let mut data = Cursor::new(vec![
        0x8f, 0x88, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf1, 0x02, 0x5f
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_int();

    assert_eq!(value, -982433);
}

#[test]
fn data_signed_int_32() {
    let mut data = Cursor::new(vec![
        0x8f, 0x88, 0xff, 0xff, 0xff, 0xff, 0xfc, 0xc5, 0xc2, 0x24
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_int();

    assert_eq!(value, -54148572);
}

#[test]
fn data_signed_int_40() {
    let mut data = Cursor::new(vec![
        0x8f, 0x88, 0xff, 0xff, 0xff, 0xf8, 0x51, 0xf8, 0x80, 0x31
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_int();

    assert_eq!(value, -32984498127);
}

#[test]
fn data_signed_int_64() {
    let mut data = Cursor::new(vec![
        0x8f, 0x88, 0xd2, 0x39, 0x8d, 0x5f, 0x0b, 0x19, 0x15, 0x66
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_int();

    assert_eq!(value, -3298449812724574874);
}

#[test]
fn data_float_32() {
    let mut data = Cursor::new(vec![
        0x8f, 0x84, 0x42, 0x2a, 0x30, 0xaf
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_float().unwrap();

    assert_eq!(value, 42.547542572021484);
}

#[test]
fn data_float_64() {
    let mut data = Cursor::new(vec![
        0x8f, 0x88, 0x40, 0x45, 0x15, 0x72, 0x23, 0xec, 0xa9, 0x59
    ]);

    let (elem, _) = reader::read_element(&mut data).unwrap();
    let value = elem.content().into_float().unwrap();

    assert_eq!(value, 42.16754578643549);
}
