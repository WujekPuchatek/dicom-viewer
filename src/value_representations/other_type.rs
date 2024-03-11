pub trait OtherType {
    type Type;

    fn new(value: std::vec::Vec<Self::Type>) -> Self;
}