use std::num::NonZeroU64;

pub trait NonZeroSized: Sized {
    const SIZE: NonZeroU64 = unsafe { NonZeroU64::new_unchecked(std::mem::size_of::<Self>() as _) };
}
/// Holds invariants? Nah!
impl<T> NonZeroSized for T where T: Sized {}