use crate::value_representations::numeric_type::NumericType;

#[derive(Debug, Clone)]
pub struct SignedLong {
    pub value: std::vec::Vec<i32>,
}

impl NumericType for SignedLong {
    type Type = i32;

    fn new(value: std::vec::Vec<i32>) -> Self {
        Self { value }
    }
}