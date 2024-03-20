use crate::dataset::data_element::DataElement;

#[derive(Debug)]
pub struct SequenceOfItems<'a> {
    pub value: std::vec::Vec<DataElement<'a>>
}