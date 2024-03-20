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
            let mut location = self.location.unwrap();
            location.reader.read_string(location.reader.unconsumed() as usize)
        }).clone()
    }
}

impl From<DataElementLocation<'_>> for DicomString
{
    fn from(v: DataElementLocation) -> Self {
        Self {
            data: OnceCell::new(),
            location: Some(v)
        }
    }
}

impl Into<String> for &DicomString
{
    fn into(self) -> String {
        self.data.get_or_init(|| {
            let location = self.location.as_ref().unwrap();
            location.reader.read_string(location.reader.unconsumed() as usize)
        }).clone()
    }
}

impl From<&DataElementLocation<'_>> for DicomString
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