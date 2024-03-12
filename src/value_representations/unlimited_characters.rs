use crate::value_representations::string_alike::StringAlike;

#[derive(Debug, Clone)]
pub struct UnlimitedCharacters {
    pub value: String,
}

impl StringAlike for UnlimitedCharacters {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}