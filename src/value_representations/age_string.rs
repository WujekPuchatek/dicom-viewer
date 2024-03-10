#[derive(Debug)]
pub struct AgeString {
    pub value: String,
}

impl AgeString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
