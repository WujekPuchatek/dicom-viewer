use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct ShortString {
    pub value: String,
}

impl StringAlike for ShortString{
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}