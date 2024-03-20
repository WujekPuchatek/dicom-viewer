use crate::dataset::tag::Tag;
use crate::data_reader::data_reader::{DataReader, Endianness};

#[derive(Clone)]
pub struct DataElementLocation
{
    pub tag: Tag,
    pub reader: DataReader,
}