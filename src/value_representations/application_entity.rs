#[derive(Debug)]
pub struct ApplicationEntity {
    pub value: String,
}

impl ApplicationEntity {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}