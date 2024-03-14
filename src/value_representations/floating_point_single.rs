use crate::value_representations::numeric_type::NumericType;

#[derive(Debug, Clone)]
pub struct FloatingPointSingle {
    pub value: std::vec::Vec<f32>
}

impl NumericType for FloatingPointSingle {
    type Type = f32;

    fn new(value: std::vec::Vec<f32>) -> Self {
        Self { value }
    }
}