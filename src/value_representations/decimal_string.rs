use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct DecimalString {
    pub value: String,
}

impl StringAlike for DecimalString {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}