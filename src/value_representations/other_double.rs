use crate::value_representations::other_type::OtherType;

#[derive(Debug)]
pub struct OtherDouble {
    pub value: std::vec::Vec<f64>,
}
impl OtherType for OtherDouble {
    type Type = f64;

    fn new(value: std::vec::Vec<f64>) -> Self {
        Self { value }
    }
}