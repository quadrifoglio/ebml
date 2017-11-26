//! The module for the document reading & parsing functionality.

use std::io::Read;
use std::collections::HashMap;

use header;
use element::{self, Element};
use error::{ErrorKind, Result};

/// Represents a read EBML element.
#[derive(Clone)]
pub struct ReadElement {
    id: element::Id,
    size: element::Size,
    children: HashMap<element::Id, ReadElement>,
    data: element::Data,
}

impl ReadElement {
    /// Returns the ID of the parsed element.
    pub fn id(&self) -> element::Id {
        self.id
    }

    /// Returns the size in bytes of the data contained within the parsed element.
    pub fn size(&self) -> element::Size {
        self.size
    }

    /// Try to find a specific element in this element's children and return a reference to it.
    pub fn find<'a, E: Element>(&'a self) -> Option<&'a ReadElement> {
        self.children.get(&E::id())
    }

    /// Returns a reference to the data contained within this element.
    pub fn data<'a>(&'a self) -> &'a element::Data {
        &self.data
    }

    /// Consumes the element object to retreive its child elements.
    pub fn children(self) -> Vec<ReadElement> {
        self.children
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
    }
}

/// A document reader. Requires a `Read` object and streams EBML elements.
pub struct Reader<R: Read> {
    reader: R,
    elements: HashMap<element::Id, bool>,
}

impl<R: Read> Reader<R> {
    /// Create a new EBML `Reader` from a `Read` object.
    fn new(reader: R) -> Reader<R> {
        let mut r = Reader {
            reader: reader,
            elements: HashMap::new(),
        };

        r.register::<header::Root>();
        r.register::<header::Version>();
        r.register::<header::ReadVersion>();
        r.register::<header::MaxIdLength>();
        r.register::<header::MaxSizeLength>();
        r.register::<header::DocType>();
        r.register::<header::DocTypeVersion>();
        r.register::<header::DocTypeReadVersion>();

        r
    }

    /// Register a new EBML element that will be recognized by the `Reader` during parsing.
    pub fn register<E: Element>(&mut self) {
        self.elements.insert(E::id(), E::is_master());
    }

    /// Read an EBML element. If `handle_data` is set to false, then the data contained within the
    /// EBML element will be ignored. It must therefore be handled by the caller (be taken
    /// off the input source) or subsequent calls to `read_element` will fail.
    pub fn read_element(&mut self, handle_data: bool) -> Result<(ReadElement, usize)> {
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

        let mut elem = ReadElement {
            id: id,
            size: size,
            children: HashMap::new(),
            data: element::Data(None),
        };

        if handle_data {
            if has_children {
                let mut r = 0 as usize;

                while r < size as usize {
                    let (child, c) = self.read_element(true)?;
                    r += c;

                    elem.children.insert(child.id(), child);
                }

                count += r;
            } else {
                let mut data = vec![0u8; size];
                let c = self.reader.read(&mut data)?;

                if c == 0 {
                    bail!(ErrorKind::UnexpectedEof);
                } else {
                    count += c;
                }

                elem.data = element::Data(Some(data));
            }
        }

        Ok((elem, count))
    }

    /// Read an EBML variable size integer (also known as a VINT). If `do_mask` is set to true,
    /// then a mask operation will be applied so that the VINT length marker bits will not be
    /// interpreted in the resulting value. Returns the value and the amout of bytes that were
    /// read.
    pub fn read_vint(&mut self, do_mask: bool) -> Result<(element::types::SignedInt, usize)> {
        let mut count = 0 as usize;
        let mut buf = [0u8; 1];

        count += self.reader.read(&mut buf)?;
        if count == 0 {
            bail!(ErrorKind::UnexpectedEof);
        }

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
            let c = self.reader.read(&mut buf[1..])?;

            if c == 0 {
                bail!(ErrorKind::UnexpectedEof);
            } else {
                count += c;
            }
        }

        for i in 0..len {
            value |= (buf[i] as i64) << ((len - i - 1) * 8);
        }

        Ok((value, count))
    }
}

impl<R: Read> ::std::convert::From<R> for Reader<R> {
    fn from(r: R) -> Reader<R> {
        Reader::new(r)
    }
}
