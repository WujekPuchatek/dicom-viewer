use crate::value_representations::numeric_type::NumericType;

#[derive(Debug, Clone)]
pub struct Signed64bitVeryLong {
    pub value: std::vec::Vec<i64>,
}

impl NumericType for Signed64bitVeryLong {
    type Type = i64;

    fn new(value: std::vec::Vec<i64>) -> Self {
        Self { value }
    }
}