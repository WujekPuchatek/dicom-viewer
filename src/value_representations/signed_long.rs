#[derive(Debug)]
pub struct SignedLong {
    pub value: i32,
}

impl SignedLong {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}