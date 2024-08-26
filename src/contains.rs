use alloc::string::String;
#[cfg(feature = "std")]
use std::path::{Path, PathBuf};

pub trait Contains<Other = Self>
where
    Other: ?Sized,
{
    fn contains(&self, other: &Other) -> bool;
}

impl Contains<str> for String {
    fn contains(&self, other: &str) -> bool {
        str::contains(self, other)
    }
}

#[cfg(feature = "std")]
impl Contains<Path> for Path {
    fn contains(&self, other: &Path) -> bool {
        other.ancestors().any(|ancestor| ancestor == self)
    }
}

#[cfg(feature = "std")]
impl<T> Contains<T> for PathBuf
where
    Path: Contains<T>,
    T: ?Sized,
{
    fn contains(&self, other: &T) -> bool {
        self.as_path().contains(other)
    }
}

#[cfg(feature = "camino_1")]
impl Contains<camino_1::Utf8Path> for camino_1::Utf8Path {
    fn contains(&self, other: &camino_1::Utf8Path) -> bool {
        self.as_std_path().contains(other.as_std_path())
    }
}

#[cfg(feature = "camino_1")]
impl<T> Contains<T> for camino_1::Utf8PathBuf
where
    camino_1::Utf8Path: Contains<T>,
    T: ?Sized,
{
    fn contains(&self, other: &T) -> bool {
        self.as_path().contains(other)
    }
}
