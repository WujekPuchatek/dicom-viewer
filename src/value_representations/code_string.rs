use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct CodeString {
    pub value: String,
}

impl StringAlike for CodeString {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}