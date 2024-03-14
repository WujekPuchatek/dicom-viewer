use crate::value_representations::numeric_type::NumericType;

#[derive(Debug, Clone)]
pub struct UnsignedShort {
    pub value: std::vec::Vec<u16>,
}

impl NumericType for UnsignedShort {
    type Type = u16;

    fn new(value: std::vec::Vec<u16>) -> Self {
        Self { value }
    }
}