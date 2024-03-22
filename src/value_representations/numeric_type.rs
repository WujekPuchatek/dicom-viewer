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