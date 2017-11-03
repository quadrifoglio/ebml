// I/O tests.

use std::io::Cursor;

use io::ReadEbml;

#[test]
fn read_vint_one_octet() {
    let vint = Cursor::new(vec![0b10001010]).read_vint().unwrap();
    assert_eq!(10, vint.value());
}

#[test]
fn read_vint_two_octets() {
    let vint = Cursor::new(vec![0x49, 0xfc]).read_vint().unwrap();
    assert_eq!(2556, vint.value());
}

#[test]
fn read_vint_four_octets() {
    let vint = Cursor::new(vec![0x12, 0x87, 0x57, 0xb2])
        .read_vint()
        .unwrap();

    assert_eq!(42424242, vint.value());
}

#[test]
fn read_vint_eight_octets() {
    let vint = Cursor::new(vec![0x01, 0x5f, 0x4d, 0x9a, 0x3c, 0x6d, 0x8e, 0x12])
        .read_vint()
        .unwrap();

    assert_eq!(26825447621627410, vint.value());
}
