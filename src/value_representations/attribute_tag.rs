#[derive(Clone, Debug)]
pub struct AttributeTag {
    pub value: Vec<[u16;2]>,
}

impl AttributeTag {
    pub fn new(value: Vec<[u16;2]>) -> Self {
        Self { value }
    }
}

