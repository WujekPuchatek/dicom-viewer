#[derive(Debug, Clone)]
pub struct UnsignedLong {
    pub value: u32,
}

impl UnsignedLong {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}