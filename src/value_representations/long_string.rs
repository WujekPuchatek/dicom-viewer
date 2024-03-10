#[derive(Debug)]
pub struct LongString {
    pub value: String,
}

impl LongString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}