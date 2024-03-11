use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct PersonName {
    pub value: String,
}

impl StringAlike for PersonName {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}