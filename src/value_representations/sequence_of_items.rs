use crate::dataset::data_element::DataElement;

#[derive(Debug, Clone)]
pub struct SequenceOfItems {
    pub value: std::vec::Vec<DataElement>
}