use crate::value_representations::other_type::OtherType;

#[derive(Debug, Clone)]
pub struct OtherByte {
    pub value: std::vec::Vec<u8>,
}
impl OtherType for OtherByte {
    type Type = u8;

    fn new(value: std::vec::Vec<u8>) -> Self {
        Self { value }
    }
}