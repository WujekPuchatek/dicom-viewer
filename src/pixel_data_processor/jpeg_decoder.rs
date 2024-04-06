use jpeg2k::*;
use crate::information_object_definitions::inconsistency::DicomFileInconsistency;
use crate::information_object_definitions::inconsistency::DicomFileInconsistency::CannotDecodeJpeg2000;

pub struct JpegFileDecoder {}

impl JpegFileDecoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode(&self, encoded: &[u8], output: &mut [u8], bytes_per_pixel: usize) -> Result<(), DicomFileInconsistency> {
        let image = Image::from_bytes(&encoded).map_err(|_| CannotDecodeJpeg2000)?;

        let components = image.components();
        let pixels = components[0].data();

        Self::save_to_output(output, pixels, bytes_per_pixel);

        Ok(())
    }

    fn save_to_output(output: &mut [u8], pixels: &[i32], bytes_per_pixel: usize) {
        pixels
            .iter()
            .enumerate()
            .for_each(|(idx, &pixel)| {
                let pixel_bytes = &pixel.to_le_bytes()[0..bytes_per_pixel];

                let output_bytes = &mut output[idx * bytes_per_pixel..(idx + 1) * bytes_per_pixel];
                output_bytes.copy_from_slice(pixel_bytes);
        });
    }

}
