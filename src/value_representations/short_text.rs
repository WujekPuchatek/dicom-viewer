use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct ShortText {
    pub value: String,
}

impl StringAlike for ShortText {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}