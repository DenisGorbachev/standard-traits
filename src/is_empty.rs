use alloc::string::String;
use alloc::vec::Vec;

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for str {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl IsEmpty for &str {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl IsEmpty for &String {
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}
impl<T> IsEmpty for [T] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<T> IsEmpty for &[T] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        <Vec<T>>::is_empty(self)
    }
}

impl<T> IsEmpty for &Vec<T> {
    fn is_empty(&self) -> bool {
        <Vec<T>>::is_empty(self)
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::*;
    use std::path::{Path, PathBuf};

    impl IsEmpty for PathBuf {
        fn is_empty(&self) -> bool {
            self.as_os_str().is_empty()
        }
    }

    impl IsEmpty for &PathBuf {
        fn is_empty(&self) -> bool {
            self.as_os_str().is_empty()
        }
    }

    impl IsEmpty for &Path {
        fn is_empty(&self) -> bool {
            self.as_os_str().is_empty()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_provide_impl_for_vec() {
        type T = Vec<i32>;
        let value: T = Vec::new();
        assert!(<T as IsEmpty>::is_empty(&value))
    }
}
