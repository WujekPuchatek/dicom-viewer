use std::rc::Rc;
use memmap2::Mmap;
use crate::utils::endianness::Endianness;

#[derive(Debug, Clone)]
pub struct Submap {
    pub file: Rc<Mmap>,
    pub start: usize,
    pub end: usize,
    pub endianness: Endianness,
}

impl Submap {
    pub fn new(file: Rc<Mmap>, start: usize, end: usize, endianness: Endianness) -> Self {
        Self { file, start, end, endianness }
    }
}