use std::fmt;

#[derive(Clone, Copy)]
pub struct AttributeTag {
    pub value: [u16;2],
}

impl AttributeTag {
    pub fn new(value: [u16;2]) -> Self {
        Self { value }
    }
}

impl fmt::Debug for AttributeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ value: [{:04x}, {:04x}] }}", self.value[0], self.value[1])
    }
}

