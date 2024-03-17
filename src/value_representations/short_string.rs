use crate::value_representations::standard_string::StringAlike;

#[derive(Debug, Clone)]
pub struct ShortString {
    pub value: String,
}

impl StringAlike for ShortString{
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}