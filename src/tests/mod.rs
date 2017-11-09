// Module containng all the tests;

use Element;

#[test]
fn elem_data_string() {
    let data = vec![0x57, 0x65, 0x73, 0x68, 0x20, 0x50, 0x6f, 0x74, 0x6f];
    let elem = Element::new(42, data.clone());

    assert_eq!(elem.get_data_utf8().unwrap(), "Wesh Poto".to_owned());
}

#[test]
fn elem_data_unsigned_integer() {
    let data = vec![0x06, 0x79, 0x32];
    let elem = Element::new(42, data.clone());

    assert_eq!(elem.get_data_i64(), 424242 as i64);
}

#[test]
fn elem_data_signed_integer() {
    let data = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x29, 0xa6];
    let elem = Element::new(42, data.clone());

    assert_eq!(elem.get_data_i64(), -54874 as i64);
}

#[test]
fn elem_data_float() {
    let data = vec![0x42, 0x28, 0xe3, 0x54];
    let elem = Element::new(42, data.clone());

    assert_eq!(elem.get_data_f32(), 42.222 as f32);
}

#[test]
fn elem_data_double() {
    let data = vec![0x40, 0xeb, 0x35, 0x8d, 0x4d, 0xad, 0xe1, 0x8b];
    let elem = Element::new(42, data.clone());

    assert_eq!(elem.get_data_f64(), 55724.415732327 as f64);
}

mod io;
