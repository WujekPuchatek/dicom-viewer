#[derive(Debug)]
pub struct ValueField {
    pub value: Box<dyn ValueRepresentation>,
}