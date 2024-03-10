#[derive(Debug)]
pub struct OtherWord {
    pub value: std::vec::Vec<i16>,
}

impl OtherWord {
    pub fn new(value: std::vec::Vec<i16>) -> Self {
        Self { value }
    }
}