use crate::information_object_definitions::inconsistency::DicomFileInconsistency;

pub struct GeneralSeries {
    modality: String,
    series_instance_uid: String,
    series_date: Option<String>,
    series_time: Option<String>,
    series_description: Option<String>,
    series_number: Option<u32>,
}

impl GeneralSeries {
    pub fn builder() -> GeneralSeriesBuilder {
        GeneralSeriesBuilder {
            modality: None,
            series_instance_uid: None,
            series_date: None,
            series_time: None,
            series_description: None,
            series_number: None,
        }
    }
}

pub struct GeneralSeriesBuilder {
    modality: Option<String>,
    series_instance_uid: Option<String>,
    series_date: Option<String>,
    series_time: Option<String>,
    series_description: Option<String>,
    series_number: Option<u32>,
}

impl GeneralSeriesBuilder {
    pub fn modality(&mut self, modality: String) -> &mut Self {
        self.modality = Some(modality);
        self
    }

    pub fn series_instance_uid(&mut self, series_instance_uid: String) -> &mut Self {
        self.series_instance_uid = Some(series_instance_uid);
        self
    }

    pub fn series_date(&mut self, series_date: String) -> &mut Self {
        self.series_date = Some(series_date);
        self
    }

    pub fn series_time(&mut self, series_time: String) -> &mut Self {
        self.series_time = Some(series_time);
        self
    }

    pub fn series_description(&mut self, series_description: String) -> &mut Self {
        self.series_description = Some(series_description);
        self
    }

    pub fn series_number(&mut self, series_number: u32) -> &mut Self {
        self.series_number = Some(series_number);
        self
    }

    pub fn build(self) -> Result<GeneralSeries, Vec<DicomFileInconsistency>> {
        self.check_for_inconsistencies()?;

        Ok(GeneralSeries {
            modality: self.modality.unwrap(),
            series_instance_uid: self.series_instance_uid.unwrap(),
            series_date: self.series_date,
            series_time: self.series_time,
            series_description: self.series_description,
            series_number: self.series_number })
    }

    fn check_for_inconsistencies(&self) -> Result<(), Vec<DicomFileInconsistency>> {
        let mut inconsistencies = Vec::new();

        if self.modality.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Modality"));
        }

        if self.series_instance_uid.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Series Instance UID"));
        }

        if !inconsistencies.is_empty() {
            return Err(inconsistencies);
        }

        Ok(())
    }
}