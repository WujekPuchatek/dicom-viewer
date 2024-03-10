#[derive(Debug)]
pub struct DecimalString {
    pub value: String,
}

impl DecimalString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}