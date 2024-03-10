#[derive(Debug)]
pub struct ShortText {
    pub value: String,
}

impl ShortText {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}