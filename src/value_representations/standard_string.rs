use std::fmt;
use once_cell::sync::Lazy;
use crate::data_reader::data_reader::DataReader;
use crate::dataset::data_element_location::DataElementLocation;

enum LazyString {
    Initialized(String),
    Uninitialized(Lazy<String>),
}

impl Clone for LazyString {
    fn clone(&self) -> Self {
        match self {
            LazyString::Initialized(value) => LazyString::Initialized(value.clone()),
            LazyString::Uninitialized(value) => LazyString::Initialized(once_cell::sync::Lazy::<String>::force(&value).clone()),
        }
    }
}

#[derive(Clone)]
pub struct StandardString
{
    data : LazyString,
}

impl From<String> for StandardString
{
    fn from(value: String) -> Self {
        Self { data: LazyString::Initialized(value) }
    }
}

impl Into<String> for StandardString
{
    fn into(self) -> String {
        match self.data {
            LazyString::Initialized(value) => value,
            LazyString::Uninitialized(value) => value.force(),
        }
    }
}

impl From<DataElementLocation> for StandardString
{
    fn from(v: DataElementLocation) -> Self {
        let read_string = || {
            let data = &v.file[v.offset as usize..(v.offset + v.length) as usize];
            let mut reader = DataReader::new(data, v.endianness);
            reader.read_string(v.length as usize)
        };

        Self {
            data: LazyString::Uninitialized(Lazy::new(read_string))
        }
    }
}

impl fmt::Debug for StandardString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ value: {} }}", self.data.into())
    }
}