use crate::value_representations::standard_string::StringAlike;

#[derive(Debug, Clone)]
pub struct ShortText {
    pub value: String,
}

impl StringAlike for ShortText {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}