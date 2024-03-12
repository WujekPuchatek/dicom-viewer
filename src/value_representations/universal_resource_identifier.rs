use crate::value_representations::string_alike::StringAlike;

#[derive(Debug, Clone)]
pub struct UniversalResourceIdentifier {
    pub value: String,
}

impl StringAlike for UniversalResourceIdentifier {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}