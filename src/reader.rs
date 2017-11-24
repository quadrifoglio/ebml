//! The module for the document reading & parsing functionality.

use std::io::Read;
use std::collections::HashMap;

use element::{self, Element};
use error::Result;

/// Represents a read EBML element.
pub struct ReadElement {
    pub id: element::Id,
    pub size: element::Size,
}

/// A document reader. Requires a `Read` object and streams EBML elements.
pub struct Reader<R: Read> {
    reader: R,
    elements: HashMap<element::Id, bool>,
}

impl<R: Read> Reader<R> {
    /// Create a new EBML Reader from a `Read` object.
    pub fn from_reader(reader: R) -> Reader<R> {
        Reader {
            reader: reader,
            elements: HashMap::new(),
        }
    }

    /// Register a new EBML element that will be recognized by the `Reader` during parsing.
    pub fn register<E: Element>(&mut self) {
        self.elements.insert(E::id(), E::has_children());
    }

    /// Read an EBML element. If `ignore_data` is set to true, then the data contained within the
    /// EBML element will not be handled. The data must therefore be handled by the caller
    /// (be taken off the input source) or subsequent calls to `read_element` will fail.
    pub fn read_element(&mut self, ignore_data: bool) -> Result<(ReadElement, element::Data, usize)> {
        let mut count = 0 as usize;

        let (id, c) = self.read_vint(false)?;
        count += c;

        let (size, c) = self.read_vint(true)?;
        count += c;

        let id = id as element::Id;
        let size = size as element::Size;

        let mut has_children = false;
        if self.elements.contains_key(&id) {
            has_children = self.elements[&id];
        }

        let elem = ReadElement { id: id, size: size };

        if !ignore_data {
            if has_children {
                let mut r = 0 as usize;

                while r < size as usize {
                    let (_, _, c) = self.read_element(false)?;
                    r += c;
                }

                // TODO: Find a way to return the children.
                return Ok((elem, element::Data::Ignored, count))
            } else {
                let mut data = vec![0u8; size];
                count += self.reader.read(&mut data)?;

                return Ok((elem, element::Data::Buffer(data), count))
            }
        } else {
            Ok((elem, element::Data::Ignored, count))
        }
    }

    /// Read an EBML variable size integer (also known as a VINT). If `do_mask` is set to true,
    /// then a mask operation will be applied so that the VINT length marker bits will not be
    /// interpreted in the resulting value. Returns the value and the amout of bytes that were
    /// read.
    pub fn read_vint(&mut self, do_mask: bool) -> Result<(element::types::SignedInt, usize)> {
        let mut count = 0 as usize;

        let mut buf = [0u8; 1];
        count += self.reader.read(&mut buf)?;

        let num = buf[0];
        let mut mask = 0x7f;
        let mut len = 1 as usize;

        if (num & 0x80) != 0 {
            len = 1;
            mask = 0x7f;
        } else if (num & 0x40) != 0 {
            len = 2;
            mask = 0x3f;
        } else if (num & 0x20) != 0 {
            len = 3;
            mask = 0x1f;
        } else if (num & 0x10) != 0 {
            len = 4;
            mask = 0x0f;
        } else if (num & 0x08) != 0 {
            len = 5;
            mask = 0x07;
        } else if (num & 0x04) != 0 {
            len = 6;
            mask = 0x03;
        } else if (num & 0x02) != 0 {
            len = 7;
            mask = 0x01;
        } else if (num & 0x01) != 0 {
            len = 8;
            mask = 0x00;
        }

        let mut value = 0 as i64;
        let mut buf = vec![0u8; len];

        if do_mask {
            buf[0] = num & mask;
        } else {
            buf[0] = num;
        }

        if len > 1 {
            count += self.reader.read(&mut buf[1..])?;
        }

        for i in 0..len {
            value |= (buf[i] as i64) << ((len - i - 1) * 8);
        }

        Ok((value, count))
    }
}
