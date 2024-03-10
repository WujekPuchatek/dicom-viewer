#[derive(Debug)]
pub struct Time {
    pub value: String,
}

impl Time {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}