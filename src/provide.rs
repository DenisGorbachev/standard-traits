#[cfg(feature = "std")]
use std::path::{Path, PathBuf};
#[cfg(feature = "syn_2")]
use syn_2::LifetimeParam;

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

#[cfg(feature = "syn_2")]
impl Provide<LifetimeParam> for syn_2::Lifetime {
    type Output = LifetimeParam;

    fn provide(self) -> LifetimeParam {
        LifetimeParam {
            attrs: vec![],
            lifetime: self,
            colon_token: None,
            bounds: Default::default(),
        }
    }
}
