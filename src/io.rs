/*!
This module contains the functionality to read/write EBML elements
to/from an I/O Reader/Writer.
*/

use std::io::{Read, Write};

use Element;
use error::Result;

/// Trait that will be implemented for every implementor of io::Read, so that it is easy to read
/// EBML elements from any kind of input.
pub trait ReadEbml {
    /// Read an EBML Element.
    fn read_ebml_element(&mut self) -> Result<Element>;
}

// Implement the ReadEbml trait for all io::Read-ers.
impl<T: Read + ?Sized> ReadEbml for T {
    fn read_ebml_element(&mut self) -> Result<Element> {
        let id = read_variable_size_integer(self)?;
        let size = read_variable_size_integer(self)? as usize;

        let mut data = vec![0u8; size];
        self.read(&mut data)?;

        Ok(Element { id: id, data: data })
    }
}

/// Trait that will be implemented for every implementor of io::Write, so that it is easy to write
/// EBML elements to any kind of output.
pub trait WriteEbml {
    /// Write an EBML Element.
    fn write_ebml_element(&mut self, elem: Element) -> Result<()>;
}

// Implement the WriteEbml trait for all io::Write-ers.
impl<T: Write + ?Sized> WriteEbml for T {
    fn write_ebml_element(&mut self, elem: Element) -> Result<()> {
        write_variable_size_integer(self, elem.id)?;
        write_variable_size_integer(self, elem.data.len() as i64)?;

        self.write(elem.data.as_ref())?;
        Ok(())
    }
}

fn read_variable_size_integer<R: Read + ?Sized>(r: &mut R) -> Result<i64> {
    let mut buf = [0u8; 1];
    r.read(&mut buf)?;

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
        r.read(&mut buf[1..])?;
    }

    for i in 0..len {
        value |= (buf[i] as i64) << ((len - i - 1) * 8);
    }

    Ok(value)
}

fn write_variable_size_integer<W: Write + ?Sized>(w: &mut W, value: i64) -> Result<()> {
    let mut mask = 0x80;
    let mut len = 1;

    if value >> 49 != 0 {
        len = 8;
        mask = 0x01;
    } else if value >> 42 != 0 {
        len = 7;
        mask = 0x02;
    } else if value >> 35 != 0 {
        len = 6;
        mask = 0x04;
    } else if value >> 28 != 0 {
        len = 5;
        mask = 0x08;
    } else if value >> 21 != 0 {
        len = 4;
        mask = 0x10;
    } else if value >> 14 != 0 {
        len = 3;
        mask = 0x20;
    } else if value >> 7 != 0 {
        len = 2;
        mask = 0x40;
    }

    let mut buf = vec![0u8; len];
    for i in 0..len {
        buf[i] = (value >> ((len - i - 1) * 8)) as u8;
    }

    buf[0] |= mask;
    w.write(buf.as_ref())?;

    Ok(())
}
