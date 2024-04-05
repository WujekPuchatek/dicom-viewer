extern crate test;

use std::ops::{Not, BitAnd, BitOr, Sub};
use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelSliceMut};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::iter::IndexedParallelIterator;
use rayon::slice::ParallelSlice;
use crate::dataset::tag::Tag;
use crate::dataset::value_field::ValueField;
use crate::dataset::value_field::ValueField::{OtherByte, OtherWord};
use crate::dicom_constants::tags::SEQUENCE_DELIMITATION;
use crate::examination::examination::Examination;
use crate::information_object_definitions::inconsistency::DicomFileInconsistency;
use crate::pixel_data_processor::jpeg_decoder::JpegFileDecoder;
use crate::value_representations::other_type::Other;

macro_rules! from_2s_complement {
    ($val:expr, $high_bit:expr) => {{
        let is_negative = $val & (1 << $high_bit);
        if is_negative != 0 {
            ($val | !((1 << ($high_bit + 1)) - 1)) as f32
        } else {
            $val as f32
        }
    }};
}
pub struct PixelDataProcessor {}

impl PixelDataProcessor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_examination(&self, exam: &Examination) -> Result<Vec<f32>, DicomFileInconsistency> {
        let dicom_files = exam.get_dicom_files();
        let image_pixel = &dicom_files[0].image_pixel;

        let num_of_pixels =
            image_pixel.columns as usize *
                image_pixel.rows as usize *
                image_pixel.samples_per_pixel as usize;

        let bits_allocated = image_pixel.bits_allocated;
        let pixel_representation = image_pixel.pixel_representation;

        let bytes_per_pixel = (bits_allocated as f32 / 8.0).ceil() as usize;
        let byte_size_of_slice = num_of_pixels * bytes_per_pixel;

        let raw_data_capacity = dicom_files.len() * byte_size_of_slice;
        let mut raw_data = vec![0; raw_data_capacity];
        let mut raw_data_chunks = raw_data.chunks_mut(byte_size_of_slice).collect::<Vec<_>>();

        let num_of_voxels = num_of_pixels * dicom_files.len();
        let mut voxels = vec![0.0f32; num_of_voxels];
        let mut voxels_chunks = voxels.chunks_mut(num_of_pixels).collect::<Vec<_>>();

        let encoded_pixels = dicom_files.iter()
            .map(|file| self.get_pixel_data(&file.image_pixel.pixel_data.value))
            .collect::<Vec<_>>();

        (&mut raw_data_chunks, &encoded_pixels, &mut voxels_chunks).into_par_iter()
            .for_each(|(slice, pixels, voxels)| {
                let slices = self.get_jpeg_encoded_data(pixels);
                self.decode_jpeg(slices, slice, bits_allocated).expect("Failed to decode JPEG2000");

                self.process_raw_values(voxels, bytes_per_pixel, slice, pixel_representation, bits_allocated);
            });

        Ok(voxels)
    }

    fn decode_jpeg(&self, slices: Vec<&[u8]>, voxels: &mut [u8], bits_allocated: u16) -> Result<(), DicomFileInconsistency> {
        if slices.len() != 1 {
            return Err(DicomFileInconsistency::NotSupported("Multiple JPEG2000 frames into one slice"));
        }

        let mut decoder = JpegFileDecoder::new();
        decoder.decode(slices[0], voxels, bits_allocated);

        Ok(())
    }

    fn process_jpeg_examination(&self, exam: &Examination) {}

    fn get_pixel_data<'a>(&self, pixel_data: &'a ValueField) -> &'a [u8] {
        match pixel_data {
            OtherByte(data) => data.as_raw_data(),
            OtherWord(data) => data.as_raw_data(),
            _ => panic!("Pixel data is not of type OW or OB")
        }
    }

    fn get_jpeg_encoded_data<'a>(&self, pixel_data: &'a [u8]) -> Vec<&'a [u8]> {
        let mut slices = Vec::new();

        let mut reader = Cursor::new(pixel_data);

        reader.seek(SeekFrom::Current(4)).unwrap();

        let offset_table_length = reader.read_u32::<LittleEndian>().unwrap() as i64;
        reader.seek(SeekFrom::Current(offset_table_length)).unwrap();

        if offset_table_length != 0 {
            let num_of_entries = offset_table_length / std::mem::size_of::<u32>() as i64;
            let num_of_slices = if num_of_entries == 0 { 1 } else { num_of_entries };

            for _ in 0..num_of_slices {
                reader.seek(SeekFrom::Current(4)).unwrap(); // Item Tag
                let item_length = reader.read_u32::<LittleEndian>().unwrap() as u64;

                let slice = &pixel_data[reader.position() as usize..
                    (reader.position() + item_length) as usize];

                slices.push(slice);

                reader.seek(SeekFrom::Current(item_length as i64)).unwrap();
            }

            reader.seek(SeekFrom::Current(4)).unwrap(); //Sequence Delimitation Item Tag
            reader.seek(SeekFrom::Current(4)).unwrap(); //Sequence Delimitation Item Length - equal to 0
        } else {
            while {
                let tag = Tag {
                    group: reader.read_u16::<LittleEndian>().unwrap(),
                    element: reader.read_u16::<LittleEndian>().unwrap()
                };

                let length = reader.read_u32::<LittleEndian>().unwrap() as u64;

                let slice = &pixel_data[reader.position() as usize..
                    (reader.position() + length) as usize];
                slices.push(slice);

                reader.seek(SeekFrom::Current(length as i64)).unwrap();
                tag != SEQUENCE_DELIMITATION
            } {}

            reader.seek(SeekFrom::Current(4)); //Sequence Delimitation Item Length - equal to 0
        }

        slices
    }

    fn read_int8(&self, voxels: &mut [f32], data: &[u8], high_bit: u16) {
        data.iter().zip(voxels.iter_mut()).for_each(|(byte, voxel)| {
            *voxel = from_2s_complement!(*byte, high_bit);
        });
    }

    fn read_uint8(&self, voxels: &mut [f32], data: &[u8], high_bit: u16) {
        data.iter().zip(voxels.iter_mut()).for_each(|(byte, voxel)| {
            *voxel = *byte as f32;
        });
    }

    fn read_int16_le(&self, voxels: &mut [f32], data: &[u8], high_bit: u16) {
        data.chunks(2).zip(voxels.iter_mut()).for_each(|(chunk, voxel)| {
            let val = i16::from_le_bytes([chunk[0], chunk[1]]);
            *voxel = from_2s_complement!(val, high_bit);
        });
    }

    fn read_uint16_le(&self, voxels: &mut [f32], data: &[u8], high_bit: u16) {
        data.chunks(2).zip(voxels.iter_mut()).for_each(|(chunk, voxel)| {
            let val = u16::from_le_bytes([chunk[0], chunk[1]]);
            *voxel = val as f32;
        });
    }

    fn process_raw_values(&self,
                          voxels: &mut [f32],
                          bytes_per_pixel: usize,
                          bytes: &[u8],
                          pixel_representation: u16,
                          high_bit: u16) {
        match pixel_representation {
            0 => match bytes_per_pixel {
                1 => self.read_uint8(voxels, bytes, high_bit),
                2 => self.read_uint16_le(voxels, bytes, high_bit),
                _ => panic!("Unsupported bytes per pixel")
            },
            1 => match bytes_per_pixel {
                1 => self.read_int8(voxels, bytes, high_bit),
                2 => self.read_int16_le(voxels, bytes, high_bit),
                _ => panic!("Unsupported bytes per pixel")
            },
            _ => panic!("Unsupported pixel representation")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_get_pixel_value(bench: &mut Bencher) {
        let processor = PixelDataProcessor::new();

        const BYTES_PER_PIXEL: usize = 2;
        const NUM_OF_PIXELS: usize = 100_000;
        const PIXEL_REPRESENTATION: u16 = 1;

        let data: Vec<u8> = vec![0x04; NUM_OF_PIXELS * BYTES_PER_PIXEL];

        bench.iter(|| {
            let chunks = data.chunks(BYTES_PER_PIXEL);

            chunks.map(|chunk| processor.get_pixel_value(chunk, BYTES_PER_PIXEL, PIXEL_REPRESENTATION))
                  .fold(0.0, |acc, x| acc + x)
        });
    }
}