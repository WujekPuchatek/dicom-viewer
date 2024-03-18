use std::fmt;
use once_cell::unsync::{OnceCell};
use crate::data_reader::data_reader::DataReader;
use crate::dataset::data_element_location::DataElementLocation;

pub struct DicomString
{
    data : OnceCell<String>,
    location: Option<DataElementLocation>
}

impl From<String> for DicomString
{
    fn from(value: String) -> Self {
        Self { data: OnceCell::with_value(value), location: None }
    }
}

impl Into<String> for DicomString
{
    fn into(self) -> String {
        self.data.get_or_init(|| {
            let location = self.location.unwrap();
            let data = &location.file[location.offset as usize..(location.offset + location.length) as usize];
            let mut reader = DataReader::new(data, location.endianness);
            reader.read_string(location.length as usize)
        }).clone()
    }
}

impl From<DataElementLocation> for DicomString
{
    fn from(v: DataElementLocation) -> Self {
        Self {
            data: OnceCell::new(),
            location: Some(v)
        }
    }
}

impl<'a> Into<String> for &'a DicomString
{
    fn into(self) -> String {
        self.data.get_or_init(|| {
            let location = self.location.as_ref().unwrap();
            let data = &location.file[location.offset as usize..(location.offset + location.length) as usize];
            let mut reader = DataReader::new(data, location.endianness);
            reader.read_string(location.length as usize)
        }).clone()
    }
}

impl From<&DataElementLocation> for DicomString
{
    fn from(v: &DataElementLocation) -> Self {
        Self {
            data: OnceCell::new(),
            location: Some(v.clone())
        }
    }
}

impl fmt::Debug for DicomString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: String = self.into();
        write!(f, "{}", value)
    }
}