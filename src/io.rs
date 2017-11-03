/*!
This module contains the functionality to read/write EBML elements
to/from an I/O Reader/Writer.
*/

use std::io::Read;

use primitives::VInt;

pub trait ReadEbml {
    fn read_vint(&mut self) -> VInt;
}
