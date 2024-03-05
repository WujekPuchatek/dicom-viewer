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

pub enum EWhence {
    Start,
    End,
    Current,
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

    pub fn read_u16(&mut self) -> u16 {
        match (self.endianness) {
            Endianness::Little => self.cursor.read_u16::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_u16::<BigEndian>().unwrap(),
        }
    }

    pub fn read_i16(&mut self) -> i16 {
        self.get_u16() as i16
    }

    pub fn read_u32(&mut self) -> u32 {
        match (self.endianness) {
            Endianness::Little => self.cursor.read_u32::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_u32::<BigEndian>().unwrap(),
        }
    }

    pub fn read_i32(&mut self) -> i32 {
        self.get_u32() as i32
    }

    pub fn read_f32(&mut self) -> f32 {
        match (self.endianness) {
            Endianness::Little => self.cursor.read_f32::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_f32::<BigEndian>().unwrap(),
        }
    }

    pub fn read_f64(&mut self) -> f64 {
        match (self.endianness) {
            Endianness::Little => self.cursor.read_f64::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_f64::<BigEndian>().unwrap(),
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0; size];
        self.cursor.read_exact(&mut buffer).unwrap();
        buffer
    }

    pub fn seek(&mut self, whence: EWhence, pos: usize) {
        match whence {
            EWhence::Start => self.cursor.set_position(pos as u64),
            EWhence::End => self.cursor.set_position((self.data.len() - pos) as u64),
            EWhence::Current => self.cursor.set_position(self.cursor.position() + pos as u64),
        }
    }
}