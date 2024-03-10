#[derive(Debug)]
pub struct DateTime {
    pub value: String,
}

impl DateTime {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}