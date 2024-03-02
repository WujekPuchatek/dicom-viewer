use crate::dataset::tag::Tag;
use crate::dataset::value_representation::ValueRepresentation;

#[derive(Debug)]
pub struct DataElement {
    pub tag: Tag,
    pub value_representation: std::option<ValueRepresentation>,
    pub value_length: u32,
    pub value: Value
}