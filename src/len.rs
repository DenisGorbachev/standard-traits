use alloc::string::String;
use alloc::vec::Vec;

#[allow(clippy::len_without_is_empty)]
pub trait Len {
    fn len(&self) -> usize;
}

impl Len for String {
    fn len(&self) -> usize {
        String::len(self)
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        Vec::<T>::len(self)
    }
}
