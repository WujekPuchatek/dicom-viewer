use crate::value_representations::other_type::OtherType;

#[derive(Debug, Clone)]
pub struct Other64bitVeryLong {
    pub value: std::vec::Vec<i64>,
}

impl Other64bitVeryLong {
    pub fn new(value: std::vec::Vec<i64>) -> Self {
        Self { value }
    }
}

impl OtherType for Other64bitVeryLong {
    type Type = i64;

    fn new(value: std::vec::Vec<i64>) -> Self {
        Self { value }
    }
}