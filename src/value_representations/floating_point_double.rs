#[derive(Debug)]
pub struct FloatingPointDouble {
    pub value: f64,
}

impl FloatingPointDouble {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}