use crate::value_representations::numeric_type::NumericType;

#[derive(Debug, Clone)]
pub struct Unsigned64bitVeryLong {
    pub value: std::vec::Vec<u64>,
}

impl NumericType for Unsigned64bitVeryLong {
    type Type = u64;

    fn new(value: std::vec::Vec<u64>) -> Self {
        Self { value }
    }
}