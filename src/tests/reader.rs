use std::io::Cursor;

use header;
use reader::Reader;

#[test]
fn ebml_header() {
    /*let mut data = Cursor::new(vec![
        0x1a, 0x45, 0xdf, 0xa3, 0x93, 0x42, 0x82, 0x88 , 0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b, 0x61,
        0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01
    ]);

    let mut r = Reader::from_reader(data);
    let elem = r.read_element::<header::Header>().unwrap();*/

    let mut data = Cursor::new(vec![
        0x42, 0x82, 0x88 , 0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b, 0x61,
        0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01
    ]);

    let mut r = Reader::from_reader(data);

    let mut doc_type = header::DocType::default();
    let dt = r.read_element(&mut doc_type).unwrap();

    let mut doc_type_ver = header::DocTypeVersion::default();
    let dt = r.read_element(&mut doc_type_ver).unwrap();

    let mut doc_type_read_ver = header::DocTypeReadVersion::default();
    let dt = r.read_element(&mut doc_type_read_ver).unwrap();
}
