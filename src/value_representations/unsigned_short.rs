#[derive(Debug, Clone)]
pub struct UnsignedShort {
    pub value: u16,
}

impl UnsignedShort {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}