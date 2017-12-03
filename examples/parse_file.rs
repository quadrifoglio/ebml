//! Usage example for the ebml-rs library.
//! Given an EBML file (for instance an MKV video), this program will print some of the values of
//! the EBML header (Version, MaxIdLength, MaxSizeLength, and DocType).

extern crate ebml;

use std::fs::File;

use ebml::header;
use ebml::reader::Reader;

fn main() {
    let path = std::env::args().nth(1).expect("Please specify a file path");

    let file = File::open(path).expect("Failed to open file");
    let mut reader = Reader::from(file);

    let (root, _) = reader.read_element().unwrap();
    let mut count = 0 as usize;

    while count < root.size() {
        let (child, c) = reader.read_element().unwrap();
        count += c;

        let data = reader.read_element_data(child.size()).unwrap();
        count += child.size();

        match child.id() {
            header::VERSION => println!("EBMLVersion: {}", data.to_unsigned_int().unwrap()),
            header::READ_VERSION => println!("EBMLReadVersion: {}", data.to_unsigned_int().unwrap()),
            header::MAX_ID_LENGTH => println!("EBMLMaxIDLength: {}", data.to_unsigned_int().unwrap()),
            header::MAX_SIZE_LENGTH => println!("EBMLMaxSizeLength: {}", data.to_unsigned_int().unwrap()),

            header::DOC_TYPE => println!("DocType: {}", data.to_utf8().unwrap().as_str()),
            header::DOC_TYPE_VERSION => println!("DocTypeVersion: {}", data.to_unsigned_int().unwrap()),
            header::DOC_TYPE_READ_VERSION => println!("DocTypeReadVersion: {}", data.to_unsigned_int().unwrap()),

            _ => {},
        };
    }
}
