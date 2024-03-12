use crate::dataset::data_element::DataElement;

#[derive(Debug, Clone)]
pub struct Unknown {
    pub value: std::vec::Vec<DataElement>,
}