use std::io::Cursor;

use Element;
use header;
use reader::Reader;

#[test]
fn ebml_header() {
    let data = Cursor::new(vec![
        0x1a, 0x45, 0xdf, 0xa3, 0x93, 0x42, 0x82, 0x88, 0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b,
        0x61, 0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01,
    ]);

    let mut r = Reader::from_reader(data);
    r.register::<header::Header>();
    r.register::<header::Version>();
    r.register::<header::ReadVersion>();
    r.register::<header::DocType>();
    r.register::<header::DocTypeVersion>();
    r.register::<header::DocTypeReadVersion>();

    let (id, _) = r.read_element(false).unwrap();
    assert_eq!(id, header::Header::id());

    let (id, _) = r.read_element(false).unwrap();
    assert_eq!(id, header::DocType::id());

    let (id, _) = r.read_element(false).unwrap();
    assert_eq!(id, header::DocTypeVersion::id());

    let (id, _) = r.read_element(false).unwrap();
    assert_eq!(id, header::DocTypeReadVersion::id());
}
