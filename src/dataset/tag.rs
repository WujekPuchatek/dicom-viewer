use std::fmt;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Tag {
    pub group: u16,
    pub element: u16,
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:#06X}, {:#06X})", self.group, self.element)
    }
}
