use crate::dataset::data_element::DataElement;

#[derive(Debug)]
pub struct Unknown<'a> {
    pub value: std::vec::Vec<DataElement<'a>>,
}