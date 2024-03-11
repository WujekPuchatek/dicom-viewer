use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct IntegerString {
    pub value: String,
}

impl StringAlike for IntegerString {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}