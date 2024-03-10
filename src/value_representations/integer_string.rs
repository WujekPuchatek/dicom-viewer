#[derive(Debug)]
pub struct IntegerString {
    pub value: String,
}

impl IntegerString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}