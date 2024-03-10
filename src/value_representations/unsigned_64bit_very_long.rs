#[derive(Debug)]
pub struct Unsigned64bitVeryLong {
    pub value: u64,
}

impl Unsigned64bitVeryLong {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}