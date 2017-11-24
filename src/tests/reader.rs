use std::io::Cursor;

use header;
use element::Element;
use reader::Reader;

#[test]
fn ebml_header_sequential() {
    let data = Cursor::new(vec![
        0x1a, 0x45, 0xdf, 0xa3, 0x93, 0x42, 0x82, 0x88, 0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b,
        0x61, 0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01,
    ]);

    let mut r = Reader::from_reader(data);
    r.register::<header::Root>();
    r.register::<header::Version>();
    r.register::<header::ReadVersion>();
    r.register::<header::DocType>();
    r.register::<header::DocTypeVersion>();
    r.register::<header::DocTypeReadVersion>();

    let (elem, _, _) = r.read_element(true).unwrap();
    assert_eq!(elem.id, header::Root::id());

    let (elem, data, _) = r.read_element(false).unwrap();
    assert_eq!(elem.id, header::DocType::id());
    assert_eq!(data.into_utf8().unwrap().as_str(), "matroska");

    let (elem, data, _) = r.read_element(false).unwrap();
    assert_eq!(elem.id, header::DocTypeVersion::id());
    assert_eq!(data.into_unsigned_int().unwrap(), 1 as u64);

    let (elem, data, _) = r.read_element(false).unwrap();
    assert_eq!(elem.id, header::DocTypeReadVersion::id());
    assert_eq!(data.into_unsigned_int().unwrap(), 1 as u64);
}
