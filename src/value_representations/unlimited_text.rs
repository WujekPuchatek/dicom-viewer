use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct UnlimitedText {
    pub value: String,
}

impl StringAlike for UnlimitedText {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}