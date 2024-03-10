#[derive(Debug)]
pub struct OtherDouble {
    pub value: std::vec::Vec<f64>,
}

impl OtherDouble {
    pub fn new(value: std::vec::Vec<f64>) -> Self {
        Self { value }
    }
}