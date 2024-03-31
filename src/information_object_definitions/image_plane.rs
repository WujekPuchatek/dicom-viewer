use crate::information_object_definitions::inconsistency::DicomFileInconsistency;

const DEFAULT_PIXEL_SPACING: [f32; 2] = [1.0, 1.0];
const DEFAULT_IMAGE_ORIENTATION: [f32; 6]  = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
const DEFAULT_IMAGE_POSITION: [f32; 3] = [0.0, 0.0, 0.0];

pub struct ImagePlane {
    pub pixel_spacing: [f32; 2],
    pub image_orientation: [f32; 6],
    pub image_position: [f32; 3],
    pub slice_location: Option<f32>,
    pub spacing_between_slices: Option<f32>,
}

impl ImagePlane {
    pub fn builder() -> ImagePlaneBuilder {
        ImagePlaneBuilder {
            pixel_spacing: None,
            image_orientation: None,
            image_position: None,
            slice_location: None,
            spacing_between_slices: None,
        }
    }
}

pub struct ImagePlaneBuilder {
    pixel_spacing: Option<[f32; 2]>,
    image_orientation: Option<[f32; 6]>,
    image_position: Option<[f32; 3]>,
    slice_location: Option<f32>,
    spacing_between_slices: Option<f32>,
}

impl ImagePlaneBuilder {
    pub fn pixel_spacing(&mut self, pixel_spacing: [f32; 2]) -> &mut Self {
        self.pixel_spacing = Some(pixel_spacing);
        self
    }

    pub fn image_orientation(&mut self, image_orientation: [f32; 6]) -> &mut Self {
        self.image_orientation = Some(image_orientation);
        self
    }

    pub fn image_position(&mut self, image_position: [f32; 3]) -> &mut Self {
        self.image_position = Some(image_position);
        self
    }

    pub fn slice_location(&mut self, slice_location: f32) -> &mut Self {
        self.slice_location = Some(slice_location);
        self
    }

    pub fn spacing_between_slices(&mut self, spacing_between_slices: f32) -> &mut Self {
        self.spacing_between_slices = Some(spacing_between_slices);
        self
    }

    pub fn build(&self) -> (ImagePlane, Vec<DicomFileInconsistency>) {
        let mut inconsistencies = Vec::new();

        if self.pixel_spacing.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Pixel Spacing"));
        }

        if self.image_orientation.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Image Orientation"));
        }

        if self.image_position.is_none() {
            inconsistencies.push(DicomFileInconsistency::MissingAttribute("Image Position"));
        }

        let image_plane = ImagePlane {
            pixel_spacing: self.pixel_spacing.unwrap_or(DEFAULT_PIXEL_SPACING),
            image_orientation: self.image_orientation.unwrap_or(DEFAULT_IMAGE_ORIENTATION),
            image_position: self.image_position.unwrap_or(DEFAULT_IMAGE_POSITION),
            slice_location: self.slice_location,
            spacing_between_slices: self.spacing_between_slices,
        };

        (image_plane, inconsistencies)
    }
}

