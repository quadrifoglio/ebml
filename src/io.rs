/*!
This module contains the functionality to read/write EBML elements
to/from an I/O Reader/Writer.
*/

use std::io::Read;

use error::Result;
use primitives::VInt;

/// Trait that will be implemented for every implementor of io::Read, so that it is easy to read
/// EBML elements from any kind of input.
pub trait ReadEbml {
    /// Read a VINT (Variable Length Integer).
    fn read_vint(&mut self) -> Result<VInt>;
}

// Implement the ReadEbml trait for all io::Read-ers.
impl<T: Read + ?Sized> ReadEbml for T {
    fn read_vint(&mut self) -> Result<VInt> {
        let mut buf = [0u8; 1];
        self.read(&mut buf)?;

        let num = buf[0];
        let mut mask = 0x00;
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
        buf[0] = num & mask;

        if len > 1 {
            self.read(&mut buf[1..])?;
        }

        for i in 0..len {
            value |= (buf[i] as i64) << ((len - i - 1) * 8);
        }

        Ok(VInt {
            length: len,
            value: value,
        })
    }
}
