pub trait PixelDataProcessor {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>>;
}