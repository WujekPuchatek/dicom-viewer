#[derive(Debug)]
pub struct UniversalResourceIdentifier {
    pub value: String,
}

impl UniversalResourceIdentifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}