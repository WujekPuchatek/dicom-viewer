use std::rc::Rc;
use memmap2::Mmap;
use crate::dataset::tag::Tag;
use crate::dataset::value_field::ValueField;
use crate::dataset::value_representation::ValueRepresentation;
use crate::data_reader::data_reader::{DataReader, Endianness};
use crate::dicom_file_parser::value_reader::ValueReader;

#[derive(Debug)]
struct DataLocation
{
    file: Rc<Mmap>,
    offset: u32,
    length: u32,
    tag: Tag,
    value_representation: Option<ValueRepresentation>
}

impl DataLocation {
    fn get_data_reader(&self) -> DataReader
    {
        DataReader::new(&self.file[self.offset as usize..(self.offset + self.length) as usize],
                        Endianness::Little)
    }

    fn get_reader(&self) -> ValueReader {
        match self.value_representation {
            Some(_) => ValueReader::new_explicit(),
            None => ValueReader::new_implicit()
        }
    }

    pub fn read_value(&self) -> Rc<ValueField>
    {
        let mut reader = self.get_data_reader();
        let value_reader = self.get_reader();
        Rc::new(value_reader.read_value(self.value_representation, self.length, &mut reader))
    }
}

#[derive(Debug)]
pub enum Value<DicomRepr> {
    DataLocation(DataLocation),
    ValueField(Rc<DicomRepr>)
}

impl<DicomRepr> Value<DicomRepr>
{
    pub fn from_value_field(value_field: DicomRepr) -> Self
    {
        Self::ValueField(Rc::new(value_field))
    }

    pub fn from_data_location(file: Rc<Mmap>,
                              offset: u32,
                              length: u32,
                              value_representation: Option<ValueRepresentation>,
                              tag: Tag) -> Self
    {
        Self::DataLocation(DataLocation { file, offset, length, tag, value_representation })
    }

    pub fn get_value_field(&mut self) -> Rc<DicomRepr>
    {
        match &self
        {
            Self::ValueField(value_field) => value_field.clone(),
            Self::DataLocation(data_location) =>
            {
                let value = data_location.read_value();
                *self = Self::ValueField(value);
                match &self
                {
                    Self::ValueField(value_field) => value_field.clone(),
                    _ => panic!("Value should be a ValueField")
                }
            }
        }
    }
}
