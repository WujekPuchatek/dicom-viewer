use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::dataset::tag::Tag;
use crate::dataset::value_field::ValueField;
use crate::dataset::value_field::ValueField::{OtherByte, OtherWord};
use crate::dicom_constants::tags::SEQUENCE_DELIMITATION;
use crate::examination::examination::Examination;
use crate::information_object_definitions::inconsistency::DicomFileInconsistency;
use crate::pixel_data_processor::jpeg_decoder::JpegFileDecoder;
use crate::value_representations::other_type::Other;

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

        let bytes_per_pixel = (image_pixel.bits_allocated as f32 / 8.0).ceil() as usize;
        let byte_size_of_slice = num_of_pixels * bytes_per_pixel;

        let raw_data_capacity = dicom_files.len() * byte_size_of_slice;

        let mut raw_data = vec![0; raw_data_capacity];

        for (idx, file) in dicom_files.iter().enumerate() {
            let slice_raw_data = &mut raw_data[idx * byte_size_of_slice..(idx + 1) * byte_size_of_slice];

            let pixels = self.get_pixel_data(&file.image_pixel.pixel_data.value);

            let encoded = self.get_jpeg_encoded_data(pixels);
            self.decode_jpeg(encoded, slice_raw_data, exam)?;
        }

        let num_of_voxels = num_of_pixels * dicom_files.len();

        let mut voxels = vec![0.0f32; num_of_voxels];

        let mut raw_data_cursor = Cursor::new(raw_data);

        for (idx, voxel) in voxels.iter_mut().enumerate() {
            let pixel = &raw_data_cursor.get_ref()[idx * bytes_per_pixel..(idx + 1) * bytes_per_pixel];

            if bytes_per_pixel == 1 {
                let value = match image_pixel.pixel_representation {
                    0 => {
                        raw_data_cursor.read_i8().unwrap() as f32
                    }
                    1 => {
                        raw_data_cursor.read_u8().unwrap() as f32
                    }
                    _ => panic!("Pixel representation is not 0 or 1")
                };


                *voxel = value;
            }
            else {
                let value = match image_pixel.pixel_representation {
                    0 => {
                        raw_data_cursor.read_i16::<LittleEndian>().unwrap() as f32
                    }
                    1 => {
                        raw_data_cursor.read_u16::<LittleEndian>().unwrap() as f32
                    }
                    _ => panic!("Pixel representation is not 0 or 1")
                };

                *voxel = value;
            }
        }

        Ok(voxels)
    }

    fn decode_jpeg(&self, slices: Vec<&[u8]>, voxels: &mut [u8], exam: &Examination) -> Result<(), DicomFileInconsistency>{
        if slices.len() != 1 {
            return Err(DicomFileInconsistency::NotSupported("Multiple JPEG2000 frames into one slice"));
        }

        let files = exam.get_dicom_files();
        let image_pixel = &files[0].image_pixel;

        let single_slice_size = image_pixel.columns as usize *
            image_pixel.rows as usize *
            image_pixel.samples_per_pixel as usize *
            image_pixel.bits_allocated as usize / 8;

        let mut decoder = JpegFileDecoder::new();

        for (idx, slice) in slices.iter().enumerate() {
            let output_slice = &mut voxels[idx * single_slice_size..(idx + 1) * single_slice_size];
            decoder.decode(slice, output_slice, image_pixel.bits_allocated);
        }

        Ok(())
    }

    fn process_jpeg_examination(&self, exam: &Examination) {

    }

    fn get_pixel_data<'a>(&self, pixel_data: &'a ValueField) -> &'a [u8] {
        match pixel_data {
            OtherByte(data) => data.as_raw_data(),
            OtherWord(data) => data.as_raw_data(),
            _ => panic!("Pixel data is not of type OW or OB")
        }
    }

    fn get_jpeg_encoded_data<'a>(&self, pixel_data: &'a [u8]) -> Vec<&'a [u8]>{
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
        }
        else {
            while {
                let tag = Tag { group: reader.read_u16::<LittleEndian>().unwrap(),
                                element: reader.read_u16::<LittleEndian>().unwrap() };

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
}