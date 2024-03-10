#[derive(Debug)]
pub struct CodeString {
    pub value: String,
}

impl CodeString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}