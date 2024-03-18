pub trait Numeric : From<Vec<Self::Type>> {
    type Type;

    fn Value(&self) -> &Vec<Self::Type>;
}

#[derive(Debug, Clone)]
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

    fn Value(&self) -> &Vec<T> {
        &self.value
    }
}