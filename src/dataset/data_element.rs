use crate::dataset::tag::Tag;
use crate::dataset::value_field::ValueField;
use crate::dataset::value_representation::ValueRepresentation;

#[derive(Debug, Clone)]
pub struct DataElement {
    pub tag: Tag,
    pub value_representation: Option<ValueRepresentation>,
    pub value_length: u32,
    pub value: ValueField,
}