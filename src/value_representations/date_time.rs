use crate::value_representations::string_alike::StringAlike;

#[derive(Debug, Clone)]
pub struct DateTime {
    pub value: String,
}

impl StringAlike for DateTime {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}