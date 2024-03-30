use crate::dataset::data_element::DataElement;
use crate::information_object_definitions::inconsistency::DicomFileInconsistency;

pub struct ImagePixel {
    pub samples_per_pixel: u16,
    pub photometric_interpretation: String,
    pub rows: u16,
    pub columns: u16,
    pub bits_allocated: u16,
    pub bits_stored: u16,
    pub high_bit: u16,
    pub pixel_representation: u16,
    pub planar_configuration: Option<u16>,
    pub pixel_data: DataElement,
}

impl ImagePixel {
    pub fn builder() -> ImagePixelBuilder {
        ImagePixelBuilder {
            samples_per_pixel: None,
            photometric_interpretation: None,
            rows: None,
            columns: None,
            bits_allocated: None,
            bits_stored: None,
            high_bit: None,
            pixel_representation: None,
            planar_configuration: None,
            pixel_data: None,
        }
    }
}

pub struct ImagePixelBuilder {
    samples_per_pixel: Option<u16>,
    photometric_interpretation: Option<String>,
    rows: Option<u16>,
    columns: Option<u16>,
    bits_allocated: Option<u16>,
    bits_stored: Option<u16>,
    high_bit: Option<u16>,
    pixel_representation: Option<u16>,
    planar_configuration: Option<Option<u16>>,
    pixel_data: Option<DataElement>,
}

impl ImagePixelBuilder {
    pub fn samples_per_pixel(mut self, samples_per_pixel: u16) -> Self {
        self.samples_per_pixel = Some(samples_per_pixel);
        self
    }

    pub fn photometric_interpretation(mut self, photometric_interpretation: String) -> Self {
        self.photometric_interpretation = Some(photometric_interpretation);
        self
    }

    pub fn rows(mut self, rows: u16) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn columns(mut self, columns: u16) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn bits_allocated(mut self, bits_allocated: u16) -> Self {
        self.bits_allocated = Some(bits_allocated);
        self
    }

    pub fn bits_stored(mut self, bits_stored: u16) -> Self {
        self.bits_stored = Some(bits_stored);
        self
    }

    pub fn high_bit(mut self, high_bit: u16) -> Self {
        self.high_bit = Some(high_bit);
        self
    }

    pub fn pixel_representation(mut self, pixel_representation: u16) -> Self {
        self.pixel_representation = Some(pixel_representation);
        self
    }

    pub fn planar_configuration(mut self, planar_configuration: Option<u16>) -> Self {
        self.planar_configuration = Some(planar_configuration);
        self
    }

    pub fn pixel_data(mut self, pixel_data: DataElement) -> Self {
        self.pixel_data = Some(pixel_data);
        self
    }

    pub fn build(self) -> Result<ImagePixel, Vec<DicomFileInconsistency>> {
        self.check_for_inconsistencies()?;

        Ok(ImagePixel {
            samples_per_pixel: self.samples_per_pixel.unwrap(),
            photometric_interpretation: self.photometric_interpretation.unwrap(),
            rows: self.rows.unwrap(),
            columns: self.columns.unwrap(),
            bits_allocated: self.bits_allocated.unwrap(),
            bits_stored: self.bits_stored.unwrap(),
            high_bit: self.high_bit.unwrap(),
            pixel_representation: self.pixel_representation.unwrap(),
            planar_configuration: self.planar_configuration.unwrap(),
            pixel_data: self.pixel_data.unwrap(),
        })
    }

    fn check_for_inconsistencies(&self) -> Result<(), Vec<DicomFileInconsistency>> {
        let mut inconsistencies = Vec::new();

        if self.samples_per_pixel.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Samples per pixel"));
        }

        if self.photometric_interpretation.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Photometric interpretation"));
        }

        if self.rows.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Rows"));
        }

        if self.columns.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Columns"));
        }

        if self.bits_allocated.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Bits allocated"));
        }

        if self.bits_stored.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Bits stored"));
        }

        if self.high_bit.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("High bit"));
        }

        if self.pixel_representation.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Pixel representation"));
        }

        if self.pixel_data.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Pixel data"));
        }

        if inconsistencies.is_empty() {
            Ok(())
        } else {
            Err(inconsistencies)
        }
    }
}