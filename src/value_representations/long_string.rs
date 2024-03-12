use crate::value_representations::string_alike::StringAlike;

#[derive(Debug, Clone)]
pub struct LongString {
    pub value: String,
}

impl StringAlike for LongString {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}