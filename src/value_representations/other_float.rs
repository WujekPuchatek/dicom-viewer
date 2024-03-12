use crate::value_representations::other_type::OtherType;

#[derive(Debug, Clone)]
pub struct OtherFloat {
    pub value: std::vec::Vec<f32>,
}
impl OtherType for OtherFloat {
    type Type = f32;

    fn new(value: std::vec::Vec<f32>) -> Self {
        Self { value }
    }
}
