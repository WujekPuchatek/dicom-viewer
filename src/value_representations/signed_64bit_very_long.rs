#[derive(Debug, Clone)]
pub struct Signed64bitVeryLong {
    pub value: i64,
}

impl Signed64bitVeryLong {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}