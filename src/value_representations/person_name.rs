#[derive(Debug)]
pub struct PersonName {
    pub value: String,
}

impl PersonName {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}