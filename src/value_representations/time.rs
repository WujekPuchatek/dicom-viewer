use crate::value_representations::string_alike::StringAlike;

#[derive(Debug, Clone)]
pub struct Time {
    pub value: String,
}

impl StringAlike for Time {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}