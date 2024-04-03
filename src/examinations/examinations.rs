use wgpu::naga::FastHashMap;
use crate::dicom_file::dicom_file::DicomFile;
use crate::examination::examination::Examination;

pub struct Examinations {
    examinations: FastHashMap<String, Examination>,
}

impl Examinations {
    pub fn new () -> Self {
        Self {
            examinations: FastHashMap::default(),
        }
    }

    pub fn add_dicom_file(&mut self, dicom_file: DicomFile) {
        let study_instance_uid  = dicom_file.general_study.study_instance_uid.clone();
        let series_instance_uid = dicom_file.general_series.series_instance_uid.clone();

        let examination_id = format!("{}-{}", study_instance_uid, series_instance_uid);

        let examination = self.examinations.entry(examination_id).or_insert(Examination::new());
        examination.add_dicom_file(dicom_file);
    }

    pub fn get_examinations(&self) -> Vec<&Examination> {
        self.examinations.values().collect()
    }
}

