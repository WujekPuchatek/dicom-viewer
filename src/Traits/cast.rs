pub trait Cast<T> {
    fn cast(&self) -> Result<T, CastError<T>>;
}

#[derive(Debug, Clone)]
pub struct CastError<T>{
    pub _type: std::marker::PhantomData<T>
}

impl<T> Default for CastError<T> {
    fn default() -> Self {
        Self { _type: std::marker::PhantomData }
    }
}

impl<T> std::fmt::Display for CastError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("Failed cast to {}", std::any::type_name::<T>()))
    }
}

#[derive(Debug, Clone)]
pub struct CastArrayError<T, const N: usize>{
    _type: std::marker::PhantomData<T>
}

impl<T, const N: usize> Default for CastArrayError<T, N> {
    fn default() -> Self {
        Self { _type: std::marker::PhantomData }
    }
}

impl<T, const N: usize> std::fmt::Display for CastArrayError<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("Failed cast to [{}; {}]", std::any::type_name::<T>(), N))
    }
}

pub trait CastArray<T, const N: usize>
{
    fn cast(&self) -> Result<[T; N], CastArrayError<T, N>>;
}