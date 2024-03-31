pub trait Cast<T> {
    fn cast(&self) -> Result<T, String>;
}

pub trait CastArray<T, const N: usize>
{
    fn cast(&self) -> Result<[T; N], String>;
}