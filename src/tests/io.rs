// I/O tests.

use std::io::Cursor;

use io::{ReadEbml, WriteEbml};

#[test]
fn vint_one_octet() {
    let data = vec![0x8a]; // 10
    let vint = Cursor::new(data.clone()).read_vint().unwrap();

    assert_eq!(10, vint);

    let mut buf = Vec::with_capacity(1);
    buf.write_vint(vint).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn vint_two_octets() {
    let data = vec![0x49, 0xfc]; // 2556
    let vint = Cursor::new(data.clone()).read_vint().unwrap();

    assert_eq!(2556, vint);

    let mut buf = Vec::with_capacity(2);
    buf.write_vint(vint).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn vint_three_octets() {
    let data = vec![0x33, 0x1f, 0x2a]; // 1253162
    let vint = Cursor::new(data.clone()).read_vint().unwrap();

    assert_eq!(1253162, vint);

    let mut buf = Vec::with_capacity(3);
    buf.write_vint(vint).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn vint_four_octets() {
    let data = vec![0x12, 0x87, 0x57, 0xb2]; // 42424242
    let vint = Cursor::new(data.clone()).read_vint().unwrap();

    assert_eq!(42424242, vint);

    let mut buf = Vec::with_capacity(4);
    buf.write_vint(vint).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn vint_five_octets() {
    let data = vec![0x0f, 0xff, 0xff, 0xff, 0xd4]; // 34359738324
    let vint = Cursor::new(data.clone()).read_vint().unwrap();

    assert_eq!(34359738324, vint);

    let mut buf = Vec::with_capacity(5);
    buf.write_vint(vint).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn vint_six_octets() {
    let data = vec![0x07, 0xff, 0xff, 0xff, 0xfe, 0x59]; // 4398046510681
    let vint = Cursor::new(data.clone()).read_vint().unwrap();

    assert_eq!(4398046510681, vint);

    let mut buf = Vec::with_capacity(6);
    buf.write_vint(vint).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn vint_seven_octets() {
    let data = vec![0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0x68]; // 562949953421160
    let vint = Cursor::new(data.clone()).read_vint().unwrap();

    assert_eq!(562949953421160, vint);

    let mut buf = Vec::with_capacity(6);
    buf.write_vint(vint).unwrap();

    assert_eq!(data, buf);
}

#[test]
fn vint_eight_octets() {
    let data = vec![0x01, 0x5f, 0x4d, 0x9a, 0x3c, 0x6d, 0x8e, 0x12]; // 26825447621627410
    let vint = Cursor::new(data.clone()).read_vint().unwrap();

    assert_eq!(26825447621627410, vint);

    let mut buf = Vec::with_capacity(6);
    buf.write_vint(vint).unwrap();

    assert_eq!(data, buf);
}
