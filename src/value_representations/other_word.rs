use crate::value_representations::other_type::OtherType;

#[derive(Debug)]
pub struct OtherWord {
    pub value: std::vec::Vec<i16>,
}

impl OtherType for OtherWord {
    type Type = i16;

    fn new(value: std::vec::Vec<i16>) -> Self {
        Self { value }
    }
}