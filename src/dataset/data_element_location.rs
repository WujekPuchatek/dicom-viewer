use std::rc::Rc;
use memmap2::Mmap;
use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::Endianness;

#[derive(Debug)]
pub struct DataElementLocation
{
    pub file: Rc<Mmap>,
    pub offset: u32,
    pub length: u32,
    pub tag: Tag,
    pub endianness: Endianness
}