use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct Date {
    pub value: String,
}

impl StringAlike for Date {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}