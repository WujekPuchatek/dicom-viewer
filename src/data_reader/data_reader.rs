extern crate byteorder;

use std::io::Read;
use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};

pub struct DataReader<'a>  {
    data: &'a [u8],
    cursor: Cursor<&'a [u8]>,
    endianness: Endianness,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}

pub enum Whence {
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

    pub fn byte_order(&self) -> Endianness {
        self.endianness
    }

    pub fn change_byte_order(&mut self, endianness : Endianness) {
        self.endianness = endianness;
    }
    pub fn read_u8(&mut self) -> u8
    {
        self.cursor.read_u8().unwrap()
    }

    pub fn read_i8(&mut self) -> i8
    {
        self.read_u8() as i8
    }

    pub fn read_u16(&mut self) -> u16 {
        match self.endianness {
            Endianness::Little => self.cursor.read_u16::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_u16::<BigEndian>().unwrap(),
        }
    }

    pub fn read_i16(&mut self) -> i16
    {
        self.read_u16() as i16
    }

    pub fn read_u32(&mut self) -> u32 {
        match self.endianness {
            Endianness::Little => self.cursor.read_u32::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_u32::<BigEndian>().unwrap(),
        }
    }

    pub fn read_i32(&mut self) -> i32
    {
        self.read_u32() as i32
    }

    pub fn read_u64(&mut self) -> u64 {
        match self.endianness {
            Endianness::Little => self.cursor.read_u64::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_u64::<BigEndian>().unwrap(),
        }
    }

    pub fn read_i64(&mut self) -> i64
    {
        self.read_u64() as i64
    }

    pub fn read_f32(&mut self) -> f32 {
        match self.endianness {
            Endianness::Little => self.cursor.read_f32::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_f32::<BigEndian>().unwrap(),
        }
    }

    pub fn read_f64(&mut self) -> f64 {
        match self.endianness {
            Endianness::Little => self.cursor.read_f64::<LittleEndian>().unwrap(),
            Endianness::Big => self.cursor.read_f64::<BigEndian>().unwrap(),
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0; size];
        self.cursor.read_exact(&mut buffer).unwrap();
        buffer
    }

    pub fn read_exact(&mut self, buffer: &mut [u8]) {
        self.cursor.read_exact(buffer).unwrap();
    }

    pub fn unconsumed(&self) -> isize {
        self.data.len() as isize - self.cursor.position() as isize
    }

    pub fn seek(&mut self, whence: Whence, pos: usize) {
        match whence {
            Whence::Start => self.cursor.set_position(pos as u64),
            Whence::End => self.cursor.set_position((self.data.len() - pos - 1) as u64),
            Whence::Current => self.cursor.set_position(self.cursor.position() + pos as u64),
        }
    }

    pub fn read_string(&mut self, size: usize) -> String {
        let buffer = self.read_bytes(size);
        String::from_utf8(buffer).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_u8(), 0x01);
    }

    #[test]
    fn test_read_i8() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_i8(), 0x01);
    }

    #[test]
    fn test_read_u16_le() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_u16(), 0x0201);
    }

    #[test]
    fn test_read_u16_be() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Big);
        assert_eq!(reader.read_u16(), 0x0102);
    }

    #[test]
    fn test_read_i16_le() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_i16(), 0x0201);
    }

    #[test]
    fn test_read_i16_be() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Big);
        assert_eq!(reader.read_i16(), 0x0102);
    }

    #[test]
    fn test_read_u32_le() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_u32(), 0x04030201);
    }

    #[test]
    fn test_read_u32_be() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Big);
        assert_eq!(reader.read_u32(), 0x01020304);
    }

    #[test]
    fn test_read_i32_le() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_i32(), 0x04030201);
    }

    #[test]
    fn test_read_i32_be() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Big);
        assert_eq!(reader.read_i32(), 0x01020304);
    }

    #[test]
    fn test_read_u64_le() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_u64(), 0x0807060504030201);
    }

    #[test]
    fn test_read_u64_be() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = DataReader::new(&data, Endianness::Big);
        assert_eq!(reader.read_u64(), 0x0102030405060708);
    }

    #[test]
    fn test_read_i64_le() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_i64(), 0x0807060504030201);
    }

    #[test]
    fn test_read_i64_be() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = DataReader::new(&data, Endianness::Big);
        assert_eq!(reader.read_i64(), 0x0102030405060708);
    }

    #[test]
    fn test_read_f32_le() {
        let data = [0x00, 0x00, 0x80, 0x3F];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_f32(), 1.0);
    }

    #[test]
    fn test_read_f32_be() {
        let data = [0x3F, 0x80, 0x00, 0x00];
        let mut reader = DataReader::new(&data, Endianness::Big);
        assert_eq!(reader.read_f32(), 1.0);
    }

    #[test]
    fn test_read_f64_le() {
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_f64(), 1.0);
    }

    #[test]
    fn test_read_f64_be() {
        let data = [0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut reader = DataReader::new(&data, Endianness::Big);
        assert_eq!(reader.read_f64(), 1.0);
    }

    #[test]
    fn test_read_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.read_bytes(4), vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_seek_start() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        reader.seek(Whence::Start, 3);
        assert_eq!(reader.read_u8(), 0x04);
    }

    #[test]
    fn test_seek_end() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);

        reader.seek(Whence::End, 0);
        assert_eq!(reader.read_u8(), 0x04);

        reader.seek(Whence::End, 1);
        assert_eq!(reader.read_u8(), 0x03);
    }

    #[test]
    fn test_seek_current() {
        let data = [0x01, 0x02, 0x03, 0x04];

        let mut reader = DataReader::new(&data, Endianness::Little);
        reader.seek(Whence::Current, 0);
        assert_eq!(reader.read_u8(), 0x01);

        let mut reader = DataReader::new(&data, Endianness::Little);
        reader.seek(Whence::Current, 1);
        assert_eq!(reader.read_u8(), 0x02);
    }

    #[test]
    fn test_read_string() {
        let data = b"test string";
        let mut reader = DataReader::new(data, Endianness::Little);
        assert_eq!(reader.read_string(11), "test string".to_string());
    }

    #[test]
    fn test_read_string_be() {
        let data = b"test string";
        let mut reader = DataReader::new(data, Endianness::Big);
        assert_eq!(reader.read_string(11), "test string".to_string());
    }

    #[test]
    fn test_unconsumed() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = DataReader::new(&data, Endianness::Little);
        assert_eq!(reader.unconsumed(), 4);

        reader.read_u8();
        assert_eq!(reader.unconsumed(), 3);

        reader.read_u16();
        assert_eq!(reader.unconsumed(), 1);
    }
}
