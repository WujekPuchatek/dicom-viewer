use crate::information_object_definitions::inconsistency::DicomFileInconsistency;

#[derive(Clone)]
pub struct ModalityLut {
    pub rescale_intercept: f32,
    pub rescale_slope: f32,
}

impl ModalityLut {
    pub fn builder() -> ModalityLutBuilder {
        ModalityLutBuilder {
            rescale_intercept: None,
            rescale_slope: None,
        }
    }
}

pub struct ModalityLutBuilder {
    rescale_intercept: Option<f32>,
    rescale_slope: Option<f32>,
}

impl ModalityLutBuilder {
    pub fn rescale_intercept(&mut self, rescale_intercept: f32) -> &mut Self {
        self.rescale_intercept = Some(rescale_intercept);
        self
    }

    pub fn rescale_slope(&mut self, rescale_slope: f32) -> &mut Self {
        self.rescale_slope = Some(rescale_slope);
        self
    }

    pub fn build(&self) -> Result<ModalityLut, Vec<DicomFileInconsistency>> {
        Ok(ModalityLut {
            rescale_intercept: self.rescale_intercept.unwrap_or(0.0),
            rescale_slope: self.rescale_slope.unwrap_or(1.0),
        })
    }
}