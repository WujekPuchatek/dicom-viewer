use std::str::FromStr;
use num_traits::Num;
use crate::traits::cast::{Cast, CastError};

pub trait Numeric : From<Vec<Self::Type>> {
    type Type;

    fn value(&self) -> &Vec<Self::Type>;
}

#[derive(Clone)]
pub struct NumericType<T> {
    value: Vec<T>,
}

impl<T> From<Vec<T>> for NumericType<T> {
    fn from(value: Vec<T>) -> Self {
        Self { value }
    }
}

impl<T> Numeric for NumericType<T> {
    type Type = T;

    fn value(&self) -> &Vec<T> {
        &self.value
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for NumericType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T: Num + FromStr + Copy> Cast<T> for NumericType<T>
{
    fn cast(&self) -> Result<T, CastError<T>> {
        let value: &Vec<T> = self.value();

        if value.is_empty() {
            return Err(CastError::<T>::default());
        }

        Ok(*value.first().unwrap())
    }
}