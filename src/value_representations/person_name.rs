use crate::value_representations::standard_string::StringAlike;

#[derive(Debug, Clone)]
pub struct PersonName {
    pub value: String,
}

impl StringAlike for PersonName {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}