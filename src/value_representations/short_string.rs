#[derive(Debug)]
pub struct ShortString {
    pub value: String,
}

impl ShortString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}