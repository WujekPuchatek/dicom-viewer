#[derive(Debug)]
pub struct UniqueIdentifier {
    pub value: String,
}

impl UniqueIdentifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}