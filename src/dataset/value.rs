use std::rc::Rc;
use memmap2::Mmap;
use crate::dataset::value_field::ValueField;

#[derive(Debug)]
struct DataLocation
{
    file: Rc<Mmap>,
    offset: u32,
    length: u32,
}

enum Value {
    DataLocation(DataLocation),
    ValueField(ValueField)
}

impl Value
{
    pub fn from_value_field(value_field: ValueField) -> Self
    {
        Self::ValueField(value_field)
    }

    pub fn from_data_location(file: Rc<Mmap>, offset: u32, length: u32) -> Self
    {
        Self::DataLocation(DataLocation { file, offset, length })
    }

    pub fn get_value_field(&mut self) -> &ValueField
    {
        match &self.data
        {
            Data::ValueField(value_field) => value_field,
            Data::DataLocation(data_location) =>
            {

            }
        }
    }
}
