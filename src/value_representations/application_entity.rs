use crate::value_representations::string_alike::StringAlike;

#[derive(Debug)]
pub struct ApplicationEntity {
    pub value: String,
}

impl StringAlike for ApplicationEntity {
    fn from_string(s: String) -> Self {
        Self { value: s }
    }
}
