use crate::value_representations::standard_string::StringAlike;

#[derive(Debug, Clone)]
pub struct AgeString {
    value: String,
}

impl StringAlike for AgeString {
    fn from(s: String) -> Self {
        Self { value: s }
    }
    fn into(self) -> String {
        self.value
    }
}
