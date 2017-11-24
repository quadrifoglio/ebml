# ebml-rs

## Description

This library aims to provide a simple and ergonomic way to read and write EBML documents.

## Example

```rust
extern crate ebml;

use std::fs::File;
use ebml::reader::Reader;

fn main() {
    let mut reader = Reader::from(File::open("sample.mkv").unwrap());

    let (header, _) = reader.read_element(true).unwrap();

    for element in header.children() {
        // Print the EBML Element ID and its size in bytes.
        println!("Element {:X} - Size {}", element.id(), element.size());
    }
}
```

## Licence

WTFPL (Basically public domain)
