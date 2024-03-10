#[derive(Debug)]
pub struct FloatingPointSingle {
    pub value: f32,
}

impl FloatingPointSingle {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}