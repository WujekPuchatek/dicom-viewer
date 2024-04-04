use std::collections::BTreeMap;
use glam::Vec3;
use crate::dicom_file::dicom_file::DicomFile;
use crate::pixel_data_processor::pixel_data_processor::PixelDataProcessor;

pub struct Examination {
    dicom_files : BTreeMap<u32, DicomFile>
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
        let dist = (z_dir.dot(position) * 1000.0) as u32;

        self.dicom_files.insert(dist, dicom_file);
    }

    pub fn get_dicom_files(&self) -> Vec<&DicomFile> {
        self.dicom_files.values().collect()
    }

    pub fn get_image_data(&self) -> Vec<f32> {
        PixelDataProcessor::new().process_examination(&self).expect("Failed to process examination")
    }
}

