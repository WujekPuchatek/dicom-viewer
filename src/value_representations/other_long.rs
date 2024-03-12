use crate::value_representations::other_type::OtherType;

#[derive(Debug, Clone)]
pub struct OtherLong {
    pub value: std::vec::Vec<i32>,
}

impl OtherType for OtherLong {
    type Type = i32;

    fn new(value: std::vec::Vec<i32>) -> Self {
        Self { value }
    }
}
