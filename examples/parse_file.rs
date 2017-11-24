//! Usage example for the ebml-rs library.
//! Given an EBML file (for instance an MKV video), this program will print some of the values of
//! the EBML header (Version, MaxIdLength, MaxSizeLength, and DocType).

extern crate ebml;

use std::fs::File;

use ebml::reader::Reader;

fn main() {
    let path = std::env::args().nth(1).expect("Please specify a file path");

    let file = File::open(path).expect("Failed to open file");
    let mut reader = Reader::from(file);

    let (header, _) = reader.read_element(true).unwrap();

    for value in header.children() {
        match value.id() {
            ebml::header::VERSION => {
                println!("Version: {}", value.data().to_unsigned_int().unwrap())
            }

            ebml::header::MAX_ID_LENGTH => {
                println!("MaxIDLength: {}", value.data().to_unsigned_int().unwrap())
            }

            ebml::header::MAX_SIZE_LENGTH => {
                println!("MaxSizeLength: {}", value.data().to_unsigned_int().unwrap())
            }

            ebml::header::DOC_TYPE => println!("EBML DocType: {}", value.data().to_utf8().unwrap()),

            _ => {}
        }
    }
}
