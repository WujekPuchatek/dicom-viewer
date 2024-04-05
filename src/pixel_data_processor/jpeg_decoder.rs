use jpeg2k::*;

pub struct JpegFileDecoder {}

impl JpegFileDecoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode(&self, encoded: &[u8], output: &mut [u8], bits_allocated: u16) {
        let bytes_per_pixel = bits_allocated as usize / 8;

        let image = Image::from_bytes(&encoded).unwrap();

        let components = image.components();

        let pixels = components[0].data();

        Self::save_to_output(output, pixels, bytes_per_pixel);
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
