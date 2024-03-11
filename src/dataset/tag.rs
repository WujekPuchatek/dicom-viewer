#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Tag {
    pub group: u16,
    pub element: u16,
}