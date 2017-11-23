//! The module for the document reading & parsing functionality.

use std::io::Read;

use ::{Element, ElementId, ElementSize};
use error::{ErrorKind, Result};
use types::SignedInt;

/// A document reader. Requires a `Read` object and streams EBML elements.
pub struct Reader<R: Read> {
	reader: R,
}

impl<R: Read> Reader<R> {
	/// Create a new EBML Reader from a `Read` object.
	pub fn from_reader(reader: R) -> Reader<R> {
		Reader { reader: reader }
	}

	/// Read an EBML element.
	pub fn read_element<E: Element>(&mut self, elem: &mut E) -> Result<()> {
        let id = self.read_vint(false)? as ElementId;
        let size = self.read_vint(true)? as ElementSize;

        let mut data = vec![0u8; size];
        self.reader.read(&mut data)?;

        if id != elem.id() {
            return Err(ErrorKind::UnexpectedElementId.into());
        }

        Ok(())
	}

	/// Read an EBML variable size integer (also known as a VINT). If `do_mask` is set to true,
	/// then a mask operation will be applied so that the VINT length marker bits will not be
	/// interpreted in the resulting value.
	pub fn read_vint(&mut self, do_mask: bool) -> Result<SignedInt> {
		let mut buf = [0u8; 1];
		self.reader.read(&mut buf)?;

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
			self.reader.read(&mut buf[1..])?;
		}

		for i in 0..len {
			value |= (buf[i] as i64) << ((len - i - 1) * 8);
		}

        Ok(value)
	}
}
