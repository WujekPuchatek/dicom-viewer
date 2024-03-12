use crate::value_representations::string_alike::StringAlike;

#[derive(Debug, Clone)]
pub struct LongText {
    pub value: String,
}

impl StringAlike for LongText {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}