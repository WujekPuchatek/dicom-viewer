use itertools::Itertools;
use crate::dataset::data_element::DataElement;
use crate::dataset::value_field::ValueField;
use crate::Traits::cast::{Cast, CastArray};
use crate::dicom_constants::tags::{PIXEL_DATA, STUDY_DATE, STUDY_INSTANCE_UID};
use crate::information_object_definitions::general_series::GeneralSeries;
use crate::information_object_definitions::general_study::GeneralStudy;
use crate::information_object_definitions::image_pixel::ImagePixel;
use crate::information_object_definitions::image_plane::ImagePlane;
use crate::information_object_definitions::inconsistency::DicomFileInconsistency;

macro_rules! get {
    ($pat: path, $target: expr, $err: expr) => {
        {
            match $target.value {
                $pat(val) => val,
                _ => {
                    $err.push(DicomFileInconsistency::UnexpectedValueRepresentation(
                        format!("Expected {} but got {:?} for tag {:?}",
                            stringify!($pat),
                            $target.value,
                            $target.tag)));
                    continue;
                }
            }
        }
    }
}

macro_rules! cast {
    ($pat: path, $target: expr, $err: expr) => {
        {
            let casted = Cast::<$pat>::cast($target);

            if casted.is_err() {
                $err.push(DicomFileInconsistency::UnexpectedValueRepresentation(casted.err().unwrap().to_string()));
                continue;
            }

            casted.unwrap()
        }
    }
}

macro_rules! cast_array {
    ($type: path, $num_of_elems: expr, $target: expr, $err: expr) => {
        {
            let casted = CastArray::<$type, $num_of_elems>::cast($target);

            if casted.is_err() {
                $err.push(DicomFileInconsistency::UnexpectedValueRepresentation(casted.err().unwrap().to_string()));
                continue;
            }

            casted.unwrap()
        }
    }
}


pub struct DicomFile {
    file_path: String,
    general_study: GeneralStudy,
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
    pub fn create(&self, file_path: &str, data_elems: Vec<DataElement>) -> Result<DicomFile, Vec<DicomFileInconsistency>> {
        let mut inconsistencies = vec![];

        let mut general_study = GeneralStudy::builder();
        let mut general_series = GeneralSeries::builder();
        let mut image_pixel = ImagePixel::builder();
        let mut image_plane = ImagePlane::builder();

        for data_elem in data_elems {
            match data_elem.tag {
                STUDY_INSTANCE_UID => {
                    let uid = &get!(ValueField::UniqueIdentifier, data_elem, inconsistencies);
                    general_study.study_instance_uid(uid.into());
                }
                STUDY_DATE => {
                    let date = &get!(ValueField::Date, data_elem, inconsistencies);
                    general_study.study_date(date.into());
                }
                MODALITY => {
                    let modality = &get!(ValueField::CodeString, data_elem, inconsistencies);
                    general_series.modality(modality.into());
                }
                SERIES_NUMBER => {
                    let number = &get!(ValueField::IntegerString, data_elem, inconsistencies);
                    general_series.series_number(cast!(u32, number, inconsistencies));
                }
                SERIES_INSTANCE_UID => {
                    let uid = &get!(ValueField::UniqueIdentifier, data_elem, inconsistencies);
                    general_series.series_instance_uid(uid.into());
                }
                SERIES_DATE => {
                    let date = &get!(ValueField::Date, data_elem, inconsistencies);
                    general_series.series_date(date.into());
                }
                SAMPLES_PER_PIXEL => {
                    let samples_per_pixel = &get!(ValueField::UnsignedShort, data_elem, inconsistencies);
                    image_pixel.samples_per_pixel(cast!(u16, samples_per_pixel, inconsistencies));
                }
                PHOTOMETRIC_INTERPRETATION => {
                    let photometric_interpretation = &get!(ValueField::CodeString, data_elem, inconsistencies);
                    image_pixel.photometric_interpretation(photometric_interpretation.into());
                }
                ROWS => {
                    let rows = &get!(ValueField::UnsignedShort, data_elem, inconsistencies);
                    image_pixel.rows(cast!(u16, rows, inconsistencies));
                }
                COLUMNS => {
                    let columns = &get!(ValueField::UnsignedShort, data_elem, inconsistencies);
                    image_pixel.columns(cast!(u16, columns, inconsistencies));
                }
                BITS_ALLOCATED => {
                    let bits_allocated = &get!(ValueField::UnsignedShort, data_elem, inconsistencies);
                    image_pixel.bits_allocated(cast!(u16, bits_allocated, inconsistencies));
                }
                BITS_STORED => {
                    let bits_stored = &get!(ValueField::UnsignedShort, data_elem, inconsistencies);
                    image_pixel.bits_stored(cast!(u16, bits_stored, inconsistencies));
                }
                HIGH_BIT => {
                    let high_bit = &get!(ValueField::UnsignedShort, data_elem, inconsistencies);
                    image_pixel.high_bit(cast!(u16, high_bit, inconsistencies));
                }
                PIXEL_REPRESENTATION => {
                    let pixel_representation = &get!(ValueField::UnsignedShort, data_elem, inconsistencies);
                    image_pixel.pixel_representation(cast!(u16, pixel_representation, inconsistencies));
                }
                PLANAR_CONFIGURATION => {
                    let planar_configuration = &get!(ValueField::UnsignedShort, data_elem, inconsistencies);
                    image_pixel.planar_configuration(Some(cast!(u16, planar_configuration, inconsistencies)));
                }
                PIXEL_DATA => {
                    image_pixel.pixel_data(data_elem);
                }

                PIXEL_SPACING => {
                    let pixel_spacing = &get!(ValueField::DecimalString, data_elem, inconsistencies);
                    image_plane.pixel_spacing(cast_array!(f32, 2, pixel_spacing, inconsistencies));
                }
                IMAGE_ORIENTATION => {
                    let image_orientation = &get!(ValueField::DecimalString, data_elem, inconsistencies);
                    image_plane.image_orientation(cast_array!(f32, 6, image_orientation, inconsistencies));
                }
                IMAGE_POSITION => {
                    let image_position = &get!(ValueField::DecimalString, data_elem, inconsistencies);
                    image_plane.image_position(cast_array!(f32, 3, image_position, inconsistencies));
                }
                SLICE_LOCATION => {
                    let slice_location = &get!(ValueField::FloatingPointSingle, data_elem, inconsistencies);
                    image_plane.slice_location(cast!(f32, slice_location, inconsistencies));
                }
                SPACING_BETWEEN_SLICES => {
                    let spacing_between_slices = &get!(ValueField::FloatingPointSingle, data_elem, inconsistencies);
                    image_plane.spacing_between_slices(cast!(f32, spacing_between_slices, inconsistencies));
                }
                _ => {}
            }
        }

        let general_study = general_study.build();
        let general_series = general_series.build();
        let image_pixel = image_pixel.build();
        let image_plane = image_plane.build();

        let inconsistensies =
            self.accumulate_inconsistencies(&general_study,
                                            &general_series,
                                            &image_pixel,
                                            &image_plane);

        if !inconsistensies.is_empty() {
            return Err(inconsistensies);
        }

        Ok(DicomFile {
            file_path: file_path.to_string(),
            general_study: general_study?,
            general_series: general_series?,
            image_pixel: image_pixel?,
            image_plane: image_plane? })
    }

    fn accumulate_inconsistencies(&self,
                                  general_study: &Result<GeneralStudy, Vec<DicomFileInconsistency>>,
                                  general_series: &Result<GeneralSeries, Vec<DicomFileInconsistency>>,
                                  image_pixel: &Result<ImagePixel, Vec<DicomFileInconsistency>>,
                                  image_plane: &Result<ImagePlane, Vec<DicomFileInconsistency>>) -> Vec<DicomFileInconsistency> {
        let mut inconsistencies = Vec::<DicomFileInconsistency>::new();

        if let Err(err) = general_study {
            inconsistencies.extend(err.to_vec());
        }

        if let Err(err) = general_series {
            inconsistencies.extend(err.clone());
        }

        if let Err(err) = image_pixel {
            inconsistencies.extend(err.clone());
        }

        if let Err(err) = image_plane {
            inconsistencies.extend(err.clone());
        }

        inconsistencies
    }
}
