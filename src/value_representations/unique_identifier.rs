use crate::value_representations::string_alike::StringAlike;

#[derive(Debug, Clone)]
pub struct UniqueIdentifier {
    pub value: String,
}

impl StringAlike for UniqueIdentifier {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}