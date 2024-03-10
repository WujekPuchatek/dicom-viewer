#[derive(Debug)]
pub struct AttributeTag {
    pub value: String,
}

impl AttributeTag {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}