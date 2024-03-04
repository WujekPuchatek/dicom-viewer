extern crate byteorder;

use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

pub struct DataReader<'a>  {
    data: &'a [u8],
    cursor: Cursor<&'a [u8]>,
    endianness: Endianness,
}

pub enum Endianness {
    Little,
    Big,
}

impl<'a> DataReader<'a> {
    pub fn new(data: &'a [u8], endianness: Endianness) -> Self {
        Self {
            data,
            cursor: std::io::Cursor::new(data),
            endianness
        }
    }
    pub fn read_u8(&mut self) -> u8 {
        match (self.endianness) {
            Endianness::Little => self.cursor.read_u8::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_u8::<BigEndian>().unwrap(),
        }
    }

    pub fn read_i8(&mut self) -> u8 {
        self.get_u8() as i8
    }
}