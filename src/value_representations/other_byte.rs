#[derive(Debug)]
pub struct OtherByte {
    pub value: std::vec::Vec<u8>,
}

impl OtherByte {
    pub fn new(value: std::vec::Vec<u8>) -> Self {
        Self { value }
    }
}