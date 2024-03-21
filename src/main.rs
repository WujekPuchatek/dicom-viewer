#![allow(dead_code)]

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

    let parser = DicomFileParser::new()
                 .file_path(path)
                 .read_tags([STUDY_DATE, STUDY_INSTANCE_UID, SERIES_INSTANCE_UID, PIXEL_DATA].as_ref())
                 .with_lazy_read_element(Some(10))
                 .parse();
    Ok(())
}