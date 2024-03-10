#[derive(Debug)]
pub struct Other64bitVeryLong {
    pub value: std::vec::Vec<i64>,
}

impl Other64bitVeryLong {
    pub fn new(value: std::vec::Vec<i64>) -> Self {
        Self { value }
    }
}