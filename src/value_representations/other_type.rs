use std::io::{Cursor, Read};
use once_cell::unsync::OnceCell;
use crate::utils::submap::Submap;

pub trait Other : From<Submap> {
    type Type;

    fn value(&self) -> &Vec<Self::Type>;
    fn as_raw_data(&self) -> &[u8];
}

#[derive(Debug, Clone)]
pub struct OtherType<T> {
    data : OnceCell<Vec<T>>,
    data_location: Submap
}

impl<T> From<Submap> for OtherType<T> {
    fn from(value: Submap) -> Self {
        Self { data: OnceCell::new(),
            data_location: value }
    }
}

impl<T> Other for OtherType<T> where T: Default + Clone {
    type Type = T;

    fn value(&self) -> &Vec<Self::Type> {
        self.data.get_or_init(|| {
            let data = self.as_raw_data();
            let bytes_length = self.data_location.end - self.data_location.start;

            let num_of_elems = bytes_length as usize / std::mem::size_of::<T>();
            let mut vec = vec![T::default(); num_of_elems as usize];

            let slice = unsafe {
                std::slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8,
                                               vec.len() * std::mem::size_of::<T>())
            };

            let mut cursor: Cursor<&[u8]> = Cursor::new(data);

            cursor.read_exact(slice).expect("Failed to read data from submap");

            vec
        })
    }

    fn as_raw_data(&self) -> &[u8] {
        &self.data_location.file[self.data_location.start..self.data_location.end]
    }
}

