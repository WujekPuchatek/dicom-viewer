use log::__private_api::Value;
use crate::dataset::data_element::DataElement;
use crate::dataset::value_field::ValueField;
use crate::dataset::value_field::ValueField::Date;
use crate::dicom_constants::tags::{PIXEL_DATA, STUDY_DATE, STUDY_INSTANCE_UID};
use crate::information_object_definitions::general_series::GeneralSeries;
use crate::information_object_definitions::general_study::GeneralStudy;
use crate::information_object_definitions::image_pixel::ImagePixel;
use crate::information_object_definitions::image_plane::ImagePlane;

macro_rules! get {
    ($data_elem:expr, $variant:pat) => {
        if let $variant(val) = $data_elem.value {
            Ok(val)
        } else {
            Err(format!("Expected {} but got {:?} for tag {:?}",
                     stringify!($variant),
                     $data_elem.value_representation,
                     $data_elem.tag))
        }
    };
}

pub struct DicomFile {
    file_path: String,
    general_series: GeneralSeries,
    image_pixel: ImagePixel,
    image_plane: ImagePlane,
}

impl DicomFile {
    pub fn factory() -> DicomFileFactory {
        DicomFileFactory {}
    }
}

pub struct DicomFileFactory {}

impl DicomFileFactory {
    pub fn create(file_path: String, data_elems: Vec<DataElement>) {
        let general_study = GeneralStudy::builder();
        let general_series = GeneralSeries::builder();
        let image_pixel = ImagePixel::builder();
        let image_plane = ImagePlane::builder();

        for data_elem in data_elems {
            if data_elem.tag == STUDY_DATE {
                let date = get!(data_elem, ValueField::Date)?;
            }
        }
    }
}
