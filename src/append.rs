use alloc::string::String;
use alloc::vec::Vec;

pub trait Append<T> {
    fn append(self, value: T) -> Self;
}

impl<T> Append<T> for Vec<T> {
    fn append(mut self, value: T) -> Self {
        <Vec<T>>::push(&mut self, value);
        self
    }
}

impl Append<&str> for String {
    fn append(mut self, value: &str) -> Self {
        self.push_str(value);
        self
    }
}

impl Append<String> for String {
    fn append(mut self, value: String) -> Self {
        self.push_str(value.as_str());
        self
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use crate::Append;
    use alloc::string::String;
    use std::ffi::{OsStr, OsString};

    impl Append<&OsStr> for OsString {
        fn append(mut self, value: &OsStr) -> Self {
            self.push(value);
            self
        }
    }

    impl Append<OsString> for OsString {
        fn append(mut self, value: OsString) -> Self {
            self.push(value.as_os_str());
            self
        }
    }

    impl Append<&str> for OsString {
        fn append(mut self, value: &str) -> Self {
            self.push(value);
            self
        }
    }

    impl Append<String> for OsString {
        fn append(mut self, value: String) -> Self {
            self.push(value.as_str());
            self
        }
    }
}
