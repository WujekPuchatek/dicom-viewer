use crate::information_object_definitions::inconsistency::DicomFileInconsistency;

#[derive(Clone)]
pub struct GeneralStudy {
    pub study_instance_uid: String,
    pub study_date: Option<String>,
    pub study_time: Option<String>,
    pub study_description: Option<String>,
    pub patient_name: Option<String>,
    pub patient_id: Option<String>,
    pub patient_birth_date: Option<String>,
}

impl GeneralStudy {
    pub fn builder() -> GeneralStudyBuilder {
        GeneralStudyBuilder {
            study_instance_uid: None,
            study_date: None,
            study_time: None,
            study_description: None,
            patient_name: None,
            patient_id: None,
            patient_birth_date: None,
        }
    }
}

pub struct GeneralStudyBuilder {
    study_instance_uid: Option<String>,
    study_date: Option<String>,
    study_time: Option<String>,
    study_description: Option<String>,
    patient_name: Option<String>,
    patient_id: Option<String>,
    patient_birth_date: Option<String>,
}

impl GeneralStudyBuilder {
    pub fn study_instance_uid(&mut self, study_instance_uid: String) -> &mut Self {
        self.study_instance_uid = Some(study_instance_uid);
        self
    }

    pub fn study_date(&mut self, study_date: String) -> &mut Self {
        self.study_date = Some(study_date);
        self
    }

    pub fn study_time(&mut self, study_time: String) -> &mut Self {
        self.study_time = Some(study_time);
        self
    }

    pub fn study_description(&mut self, study_description: String) -> &mut Self {
        self.study_description = Some(study_description);
        self
    }

    pub fn patient_name(&mut self, patient_name: String) -> &mut Self {
        self.patient_name = Some(patient_name);
        self
    }

    pub fn patient_id(&mut self, patient_id: String) -> &mut Self {
        self.patient_id = Some(patient_id);
        self
    }

    pub fn patient_birth_date(&mut self, patient_birth_date: String) -> &mut Self {
        self.patient_birth_date = Some(patient_birth_date);
        self
    }

    pub fn build(&self) -> Result<GeneralStudy, Vec<DicomFileInconsistency>> {
        self.check_for_inconsistencies()?;

        Ok(GeneralStudy {
            study_instance_uid: self.study_instance_uid.clone().unwrap(),
            study_date: self.study_date.clone(),
            study_time: self.study_time.clone(),
            study_description: self.study_description.clone(),
            patient_name: self.patient_name.clone(),
            patient_id: self.patient_id.clone(),
            patient_birth_date: self.patient_birth_date.clone(),
        })
    }

    fn check_for_inconsistencies(&self) -> Result<(), Vec<DicomFileInconsistency>> {
        let mut inconsistencies = Vec::new();

        if self.study_instance_uid.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Study Instance UID"));
        }

        if !inconsistencies.is_empty() {
            return Err(inconsistencies);
        }

        Ok(())
    }
}