#[cfg(feature = "std")]
use std::path::{Path, PathBuf};

/// `Provide` is similar to `AsRef`, `Into`, `TryInto`. However, `Provide` allows specifying the return type, so the implementor may choose to return a `T`, `&T`, `Option<T>`, `Result<T, ...>`.
///
/// `Provide` is frequently used with [`Get`] (see its documentation for details).
pub trait Provide<T: ?Sized> {
    type Output;

    fn provide(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Provide<PathBuf> for &Path {
    type Output = PathBuf;

    fn provide(self) -> PathBuf {
        self.to_path_buf()
    }
}
