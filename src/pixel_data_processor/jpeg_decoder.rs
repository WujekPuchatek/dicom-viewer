use std::io;
use std::io::{Cursor, Seek, SeekFrom, Write};
use jpeg2k::*;
use crate::information_object_definitions::inconsistency::DicomFileInconsistency;
use crate::information_object_definitions::inconsistency::DicomFileInconsistency::CannotDecodeJpeg2000;

pub struct JpegFileDecoder {}

impl JpegFileDecoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode(&self, encoded: &[u8], output: &mut [u8], bits_allocated: u16) -> Result<(), DicomFileInconsistency> {
        let bytes_per_pixel = bits_allocated as usize / 8;

        let image = Image::from_bytes(&encoded).map_err(|e| CannotDecodeJpeg2000)?;

        let components = image.components();
        if components.len() != 1 {
            return Err(CannotDecodeJpeg2000);
        }

        let mut output_cursor = Cursor::new(output);

        for pixel in components[0].data().iter() {
            output_cursor.write_all(&pixel.to_le_bytes()[0..bytes_per_pixel]).map_err(|_| CannotDecodeJpeg2000)?;
            output_cursor.seek(SeekFrom::Current(bytes_per_pixel as i64)).map_err(|_| CannotDecodeJpeg2000)?;
        }

        Ok(())
    }
}

