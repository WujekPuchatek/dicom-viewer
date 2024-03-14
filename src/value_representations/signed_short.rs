use crate::value_representations::numeric_type::NumericType;

#[derive(Debug, Clone)]
pub struct SignedShort {
    pub value: std::vec::Vec<i16>,
}

impl NumericType for SignedShort {
    type Type = i16;

    fn new(value: std::vec::Vec<i16>) -> Self {
        Self { value }
    }
}