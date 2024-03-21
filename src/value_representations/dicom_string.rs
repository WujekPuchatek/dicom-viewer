use std::fmt;
use once_cell::unsync::{OnceCell};
use crate::dataset::data_element_location::DataElementLocation;

pub struct DicomString
{
    data : OnceCell<String>,
    location: Option<DataElementLocation<String>>
}

impl From<String> for DicomString
{
    fn from(value: String) -> Self {
        Self { data: OnceCell::with_value(value),
               location: None }
    }
}

impl Into<String> for &DicomString
{
    fn into(self) -> String {
        self.data.get_or_init(|| {
            self.location.as_ref().expect("REASON").read_value() }).to_string()
    }
}

impl From<DataElementLocation<String>> for DicomString
{
    fn from(v: DataElementLocation<String>) -> Self {
        Self {
            data: OnceCell::new(),
            location: Some(v)
        }
    }
}

impl fmt::Debug for DicomString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: String = self.into();
        write!(f, "{}", value)
    }
}