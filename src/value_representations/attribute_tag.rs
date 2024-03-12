use crate::value_representations::string_alike::StringAlike;

#[derive(Debug, Clone)]
pub struct AttributeTag {
    pub value: String,
}

impl StringAlike for AttributeTag {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}