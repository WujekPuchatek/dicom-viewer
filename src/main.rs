#![allow(dead_code)]

use std::io::{Error as OtherError, ErrorKind};
use crate::dataset::tag::Tag;
use crate::dicom_constants::tags::{PIXEL_DATA, SERIES_INSTANCE_UID, STUDY_DATE, STUDY_INSTANCE_UID};
use crate::dicom_file_parser::dicom_file_parser::DicomFileParser;

mod data_reader;
mod dicom_file_parser;
mod dicom_constants;
mod dataset;
mod value_representations;
mod utils;

fn main()  -> std::io::Result<()>
{
    let path = "C:/Users/medapp/Desktop/CT/0015.dcm";

    pub const IMAGE_POSITION: Tag = Tag { group: 0x0020, element: 0x0032 };
    pub const IMAGE_ORIENTATION: Tag = Tag { group: 0x0020, element: 0x0037 };
    pub const SAMPLES_PER_PIXEL: Tag = Tag { group: 0x0028, element: 0x0002 };
    pub const PHOTOMETRIC_INTERPRETATION: Tag = Tag { group: 0x0028, element: 0x0004 };
    pub const ROWS: Tag = Tag { group: 0x0028, element: 0x0010 };
    pub const COLUMNS: Tag = Tag { group: 0x0028, element: 0x0011 };
    pub const PIXEL_SPACING: Tag = Tag { group: 0x0028, element: 0x0030 };
    pub const BITS_ALLOCATED: Tag = Tag { group: 0x0028, element: 0x0100 };
    pub const BITS_STORED: Tag = Tag { group: 0x0028, element: 0x0101 };
    pub const HIGH_BIT: Tag = Tag { group: 0x0028, element: 0x0102 };
    pub const PIXEL_REPRESENTATION: Tag = Tag { group: 0x0028, element: 0x0103 };
    pub const WINDOW_CENTER: Tag = Tag { group: 0x0028, element: 0x1050 };
    pub const WINDOW_WIDTH: Tag = Tag { group: 0x0028, element: 0x1051 };
    pub const RESCALE_INTERCEPT: Tag = Tag { group: 0x0028, element: 0x1052 };
    pub const RESCALE_SLOPE: Tag = Tag { group: 0x0028, element: 0x1053 };

    let tags_to_read = [
        STUDY_DATE,
        STUDY_INSTANCE_UID,
        IMAGE_POSITION,
        IMAGE_ORIENTATION,
        SAMPLES_PER_PIXEL,
        PHOTOMETRIC_INTERPRETATION,
        ROWS,
        COLUMNS,
        PIXEL_SPACING,
        BITS_ALLOCATED,
        BITS_STORED,
        HIGH_BIT,
        PIXEL_REPRESENTATION,
        WINDOW_CENTER,
        WINDOW_WIDTH,
        RESCALE_INTERCEPT,
        RESCALE_SLOPE,
        PIXEL_DATA].as_ref();

    let dicom_data_elems = DicomFileParser::new()
                 .file_path(path)
                 .read_tags(tags_to_read)
                 .with_lazy_read_element(Some(10))
                 .parse();

    if let Err(e) = dicom_data_elems {
        println!("Error: {}", e);
        return Err(std::io::Error::new(ErrorKind::Other, "An error occurred"));
    }

    let data_elems = dicom_data_elems.unwrap();
    for elem in data_elems {
        println!("{:?}", elem);
    }

    Ok(())
}