#[derive(Debug)]
pub struct Date {
    pub value: String,
}

impl Date {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}