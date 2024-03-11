use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct AgeString {
    pub value: String,
}

impl StringAlike for AgeString {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}
