//! This module provides the functionality to read EBML documents.

use std::io::Read;

use common::types::*;
use header::{self, Header};
use common::{Element, ElementContent};
use error::{Error, ErrorKind, Result};

/// Read the standard EBML header.
pub fn read_header<R: Read>(r: &mut R) -> Result<(Header, usize)> {
    let mut header = Header::default();

    let (elem, c) = read_element(r)?;
    let mut root = elem.content().children()?;

    header.version = root.find(header::VERSION)
        .map_or(1, |elem| elem.content().into_uint());

    header.read_version = root.find(header::READ_VERSION)
        .map_or(1, |elem| elem.content().into_uint());

    header.max_id_length = root.find(header::MAX_ID_LENGTH)
        .map_or(4, |elem| elem.content().into_uint());

    header.max_size_length = root.find(header::MAX_SIZE_LENGTH)
        .map_or(8, |elem| elem.content().into_uint());

    header.doc_type = root.find(header::DOC_TYPE)
        .ok_or(Error::from(ErrorKind::ElementNotFound(header::DOC_TYPE)))?
        .content().into_utf8()?;

    header.doc_type_version = root.find(header::DOC_TYPE_VERSION)
        .map_or(1, |elem| elem.content().into_uint());

    header.doc_type_read_version = root.find(header::DOC_TYPE_READ_VERSION)
        .map_or(1, |elem| elem.content().into_uint());

    Ok((header, c))
}

/// Read an entire EBML element.
pub fn read_element<R: Read>(r: &mut R) -> Result<(Element, usize)> {
    let mut count = 0 as usize;

    let (id, size, c) = read_element_info(r)?;
    count += c;

    let (content, c) = read_element_data(r, size)?;
    count += c;

    let elem = Element {
        id: id,
        size: size,
        content: content
    };

    Ok((elem, count))
}

/// Read the information about an EBML element. That information consists of an ID and the size of
/// the data that the element contains.
pub fn read_element_info<R: Read>(r: &mut R) -> Result<(ElementId, ElementSize, usize)> {
    let mut count = 0 as usize;

    let (id, c) = read_vint(r, false)?;
    count += c;

    let (size, c) = read_vint(r, true)?;
    count += c;

    Ok((id as ElementId, size as ElementSize, count))
}

/// Read the data contained in an EBML element.
pub fn read_element_data<R: Read>(r: &mut R, size: ElementSize) -> Result<(ElementContent, usize)> {
    let mut buf = vec![0u8; size];
    let count = r.read(&mut buf)?;

    Ok((ElementContent::new(buf), count))
}

/// Read an EBML variable size integer (also known as a VINT). If `do_mask` is set to true,
/// then a mask operation will be applied so that the VINT length marker bits will not be
/// interpreted in the resulting value. Returns the value and the amout of bytes that were
/// read.
pub fn read_vint<R: Read>(r: &mut R, do_mask: bool) -> Result<(SignedInt, usize)> {
    let mut count = 0 as usize;
    let mut buf = [0u8; 1];

    count += r.read(&mut buf)?;
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
        let c = r.read(&mut buf[1..])?;

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
