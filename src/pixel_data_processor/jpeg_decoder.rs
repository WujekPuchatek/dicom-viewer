use std::error::Error;

pub trait JpegDecoder {
    fn decode(&self, source: &[u8], dest: &[u8]) -> Result<dyn Error> {
        
    }
}