#[derive(Debug)]
pub struct UnlimitedText {
    pub value: String,
}

impl UnlimitedText {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}