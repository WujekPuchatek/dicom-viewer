#[derive(Debug)]
pub struct OtherLong {
    pub value: std::vec::Vec<i32>,
}

impl OtherLong {
    pub fn new(value: std::vec::Vec<i32>) -> Self {
        Self { value }
    }
}
