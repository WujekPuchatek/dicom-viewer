pub trait NumericType {
    type Type;

    fn new(value: std::vec::Vec<Self::Type>) -> Self;
}