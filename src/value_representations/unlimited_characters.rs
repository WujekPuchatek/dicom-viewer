#[derive(Debug)]
pub struct UnlimitedCharacters {
    pub value: String,
}

impl UnlimitedCharacters {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}