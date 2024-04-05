use std::collections::BTreeMap;
use glam::Vec3;
use web_sys::js_sys::Math::abs;
use crate::dicom_file::dicom_file::DicomFile;
use crate::pixel_data_processor::pixel_data_processor::PixelDataProcessor;
use crate::utils::data_dimensions::Dimensions;

pub struct Examination {
    dicom_files : BTreeMap<i32, DicomFile>
}

impl Examination {
    pub fn new() -> Self {
        Self {
            dicom_files: BTreeMap::new()
        }
    }

    pub fn add_dicom_file(&mut self, dicom_file: DicomFile) {
        let orientation = &dicom_file.image_plane.image_orientation;
        let x_dir = Vec3::from_slice(&orientation[0..3]);
        let y_dir = Vec3::from_slice(&orientation[3..6]);
        let z_dir = x_dir.cross(y_dir);

        let position = Vec3::from_slice(&dicom_file.image_plane.image_position);
        let dist = (z_dir.dot(position) * 1000.) as i32;

        self.dicom_files.insert(dist, dicom_file);
    }

    pub fn get_dicom_files(&self) -> Vec<&DicomFile> {
        self.dicom_files.values().collect()
    }

    pub fn get_image_data(&self) -> Vec<f32> {
        PixelDataProcessor::new().process_examination(&self).expect("Failed to process examination")
    }

    pub fn get_dimensions(&self) -> Dimensions {
        let (first_slice_pos, first_file) = self.dicom_files.iter().next().unwrap();
        let (last_slice_pos, _) = self.dicom_files.iter().next_back().unwrap();

        let image_plane = &first_file.image_plane;
        let image_pixel = &first_file.image_pixel;
        let num_files = self.dicom_files.len();

        let dst_between_slices = (last_slice_pos - first_slice_pos).abs() as f32 / (num_files - 1) as f32 / 1000.0;

        let mut builder = Dimensions::builder();
        builder.width(image_pixel.columns as u32)
               .height(image_pixel.rows as u32)
               .depth(self.dicom_files.len() as u32)
               .pixel_spacing(image_plane.pixel_spacing)
               .distance_between_slices(dst_between_slices)
               .build()

    }
}

