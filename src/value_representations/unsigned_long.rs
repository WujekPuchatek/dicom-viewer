use crate::value_representations::numeric_type::NumericType;

#[derive(Debug, Clone)]
pub struct UnsignedLong {
    pub value: std::vec::Vec<u32>,
}

impl NumericType for UnsignedLong {
    type Type = u32;

    fn new(value: std::vec::Vec<u32>) -> Self {
        Self { value }
    }
}
