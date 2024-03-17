use std::fmt;
use once_cell::sync::Lazy;
use crate::dataset::data_element_location::DataElementLocation;
use crate::value_representations::standard_string::StandardString;

#[derive(Clone)]
enum LazyString {
    Initialized(String),
    Uninitialized(Lazy<String>),
}

#[derive(Clone)]
pub struct ExtendedString
{
    data : LazyString,
}

impl From<String> for ExtendedString
{
    fn from(value: String) -> Self {
        Self { data: LazyString::Initialized(value) }
    }
}

impl Into<String> for ExtendedString
{
    fn into(self) -> String {
        match self.data {
            LazyString::Initialized(value) => value,
            LazyString::Uninitialized(value) => value.force(),
        }
    }
}

impl From<DataElementLocation> for ExtendedString
{
    fn from(v: DataElementLocation) -> Self {
        let read_string = || {
            StandardString::from(v).into()
        };

        Self {
            data: LazyString::Uninitialized(Lazy::new(read_string))
        }
    }
}

impl fmt::Debug for ExtendedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ value: {} }}", self.data.into())
    }
}