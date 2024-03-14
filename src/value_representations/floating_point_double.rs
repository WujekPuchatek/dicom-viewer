use crate::value_representations::numeric_type::NumericType;

#[derive(Debug, Clone)]
pub struct FloatingPointDouble {
    pub value: std::vec::Vec<f64>,
}

impl NumericType for FloatingPointDouble {
    type Type = f64;

    fn new(value: std::vec::Vec<f64>) -> Self {
        Self { value }
    }
}