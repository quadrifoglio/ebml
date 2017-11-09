// I/O tests.

use std::io::Cursor;

use Element;
use io::{ReadEbml, WriteEbml};

#[test]
fn element_write_with_data_one_octet() {
    let data = vec![0xaa, 0x81, 10];
    let elem = Element::new(42, vec![10]);

    let mut buf = Vec::with_capacity(3);
    buf.write_ebml_element(elem).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn element_read_with_data_one_octet() {
    let mut data = Cursor::new(vec![0xaa, 0x81, 10]);
    let elem = data.read_ebml_element().unwrap();

    assert_eq!(42, elem.id());
    assert_eq!(1, elem.data().len());
    assert_eq!(&vec![10], elem.data());
}

#[test]
fn element_write_with_data_one_megs() {
    let mut data = vec![0xaa, 0x30, 0x00, 0x00];
    data.extend(vec![42u8; 0x100000]); // 1 MiB of data

    let elem = Element::new(42, vec![42u8; 0x100000]);

    let mut buf = Vec::with_capacity(3);
    buf.write_ebml_element(elem).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn element_read_with_data_one_megs() {
    let mut data = vec![0xaa, 0x30, 0x00, 0x00];
    data.extend(vec![42u8; 0x100000]); // 1 MiB of data

    let mut data = Cursor::new(data);
    let elem = data.read_ebml_element().unwrap();

    assert_eq!(42, elem.id());
    assert_eq!(0x100000, elem.data().len());
    assert_eq!(&vec![42u8; 0x100000], elem.data());
}
