#[derive(Debug)]
pub struct SignedShort {
    pub value: i16,
}

impl SignedShort {
    pub fn new(value: i16) -> Self {
        Self { value }
    }
}