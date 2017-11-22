//! The module for the document reading & parsing functionality.

use std::io::Read;

/// A document reader. Requires a `Read` object and streams EBML elements.
pub struct Reader<R: Read> {
    reader: R,
}

impl<R: Read> Reader<R> {
    /// Create a new EBML Reader from a `Read` object.
    pub fn from_reader(reader: R) -> Reader<R> {
        Reader { reader: reader }
    }
}
