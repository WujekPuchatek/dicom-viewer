use std::fmt;
use std::str::FromStr;
use num_traits::Num;
use crate::dataset::data_element_location::DataElementLocation;
use crate::traits::cast::{Cast, CastArray, CastArrayError, CastError};
use crate::value_representations::dicom_string::DicomString;
pub struct NumericString {
    data: DicomString
}
impl Into<String> for NumericString {
    fn into(self) -> String {
        (&self).into()
    }
}

impl Into<String> for &NumericString {
    fn into(self) -> String {
        (&self.data).into()
    }
}

impl From<String> for NumericString {
    fn from(value: String) -> Self {
        Self { data: DicomString::from(value) }
    }
}

impl From<DataElementLocation<String>> for NumericString {
    fn from(v: DataElementLocation<String>) -> Self {
        Self { data: DicomString::from(v) }
    }
}



impl<T: Num + FromStr> Cast<T> for NumericString {
    fn cast(&self) -> Result<T, CastError<T>> {
        let str: String = self.into();

        match str.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => Err(CastError::<T>::default())
        }
    }
}

impl<T: Num + FromStr + Copy + Default, const N: usize> CastArray<T, N> for NumericString
    where <T as FromStr>::Err: std::fmt::Debug {
    fn cast(&self) -> Result<[T; N], CastArrayError<T, N>> {
        let str: String = self.into();
        let values: Vec<T> = str.split("\\").map(|s| s.parse::<T>().unwrap()).collect();

        if values.len() != N {
            return Err(CastArrayError::<T, N>::default());
        }

        let mut arr: [T; N] = [Default::default(); N];
        arr.copy_from_slice(&values);

        Ok(arr)
    }
}

impl fmt::Debug for NumericString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: String = self.into();
        write!(f, "{}", value)
    }
}
