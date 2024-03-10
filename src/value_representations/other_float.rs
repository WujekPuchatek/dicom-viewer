#[derive(Debug)]
pub struct OtherFloat {
    pub value: std::vec::Vec<f32>,
}

impl OtherFloat {
    pub fn new(value: std::vec::Vec<f32>) -> Self {
        Self { value }
    }
}