use itertools::Itertools;
use crate::dataset::data_element::DataElement;
use crate::dataset::value_field::ValueField;
use crate::Traits::cast::{Cast, CastArray};
use crate::dicom_constants::tags::{PIXEL_DATA, STUDY_DATE, STUDY_INSTANCE_UID};
use crate::information_object_definitions::general_series::GeneralSeries;
use crate::information_object_definitions::general_study::GeneralStudy;
use crate::information_object_definitions::image_pixel::ImagePixel;
use crate::information_object_definitions::image_plane::ImagePlane;

macro_rules! get {
    ($target: expr, $pat: path) => {
        {
            if let $pat(val) = $target.value { // #1
                Ok(val)
            } else {
                Err(format!("Expected {} but got {:?} for tag {:?}",
                    stringify!($pat),
                    $target.value_representation,
                    $target.tag))
            }
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
    pub fn create(file_path: String, data_elems: Vec<DataElement>) -> Result<(), String> {
        let mut general_study = GeneralStudy::builder();
        let mut general_series = GeneralSeries::builder();
        let mut image_pixel = ImagePixel::builder();
        let mut image_plane = ImagePlane::builder();

        for data_elem in data_elems {
            match data_elem.tag {
                STUDY_INSTANCE_UID => {
                    let uid = &get!(data_elem, ValueField::UniqueIdentifier)?;
                    general_study.study_instance_uid(uid.into());
                }
                STUDY_DATE => {
                    let date = &get!(data_elem, ValueField::Date)?;
                    general_study.study_date(date.into());
                }
                MODALITY => {
                    let modality = &get!(data_elem, ValueField::CodeString)?;
                    general_series.modality(modality.into());
                }
                SERIES_NUMBER => {
                    let number = &get!(data_elem, ValueField::IntegerString)?;
                    general_series.series_number(Cast::<u32>::cast(number)?);
                }
                SERIES_INSTANCE_UID => {
                    let uid = &get!(data_elem, ValueField::UniqueIdentifier)?;
                    general_series.series_instance_uid(uid.into());
                }
                SERIES_DATE => {
                    let date = &get!(data_elem, ValueField::Date)?;
                    general_series.series_date(date.into());
                }
                SAMPLES_PER_PIXEL => {
                    let samples_per_pixel = &get!(data_elem, ValueField::UnsignedShort)?;
                    image_pixel.samples_per_pixel(Cast::<u16>::cast(samples_per_pixel)?);
                }
                PHOTOMETRIC_INTERPRETATION => {
                    let photometric_interpretation = &get!(data_elem, ValueField::CodeString)?;
                    image_pixel.photometric_interpretation(photometric_interpretation.into());
                }
                ROWS => {
                    let rows = &get!(data_elem, ValueField::UnsignedShort)?;
                    image_pixel.rows(Cast::<u16>::cast(rows)?);
                }
                COLUMNS => {
                    let columns = &get!(data_elem, ValueField::UnsignedShort)?;
                    image_pixel.columns(Cast::<u16>::cast(columns)?);
                }
                BITS_ALLOCATED => {
                    let bits_allocated = &get!(data_elem, ValueField::UnsignedShort)?;
                    image_pixel.bits_allocated(Cast::<u16>::cast(bits_allocated)?);
                }
                BITS_STORED => {
                    let bits_stored = &get!(data_elem, ValueField::UnsignedShort)?;
                    image_pixel.bits_stored(Cast::<u16>::cast(bits_stored)?);
                }
                HIGH_BIT => {
                    let high_bit = &get!(data_elem, ValueField::UnsignedShort)?;
                    image_pixel.high_bit(Cast::<u16>::cast(high_bit)?);
                }
                PIXEL_REPRESENTATION => {
                    let pixel_representation = &get!(data_elem, ValueField::UnsignedShort)?;
                    image_pixel.pixel_representation(Cast::<u16>::cast(pixel_representation)?);
                }
                PLANAR_CONFIGURATION => {
                    let planar_configuration = &get!(data_elem, ValueField::UnsignedShort)?;
                    image_pixel.planar_configuration(Some(Cast::<u16>::cast(planar_configuration)?));
                }
                PIXEL_DATA => {
                    image_pixel.pixel_data(data_elem);
                }

                PIXEL_SPACING => {
                    let pixel_spacing = &get!(data_elem, ValueField::DecimalString)?;
                    let pixel_spacing = CastArray::<f32, 2>::cast(pixel_spacing)?;

                    image_plane.pixel_spacing(pixel_spacing);
                }
                IMAGE_ORIENTATION => {
                    let image_orientation = &get!(data_elem, ValueField::DecimalString)?;
                    image_plane.image_orientation(CastArray::<f32,6>::cast(image_orientation)?);
                }
                IMAGE_POSITION => {
                    let image_position = &get!(data_elem, ValueField::DecimalString)?;
                    image_plane.image_position(CastArray::<f32, 3>::cast(image_position)?);
                }
                SLICE_LOCATION => {
                    let slice_location = &get!(data_elem, ValueField::FloatingPointSingle)?;
                    image_plane.slice_location(Cast::<f32>::cast(slice_location)?);
                }

                SPACING_BETWEEN_SLICES => {
                    let spacing_between_slices = &get!(data_elem, ValueField::FloatingPointSingle)?;
                    image_plane.spacing_between_slices(Cast::<f32>::cast(spacing_between_slices)?);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
