use crate::Provide;

/// `Get` trait is a replacement for a simple getter function. For example: instead of writing `get_project_root(&path)`, you can write `path.get::<ProjectRoot>()`.
///
/// `Get` relies on [`Provide`]: every type that implements [`Provide`] also implements `Get` via a blanket implementation. Please implement [`Provide`] instead of `Get` for your types.
///
/// # Comparison
///
/// * [Implementation via fn](#implementation-via-fn)
/// * [Implementation via `Get`](#implementation-via-get)
///
/// ## Implementation via fn
///
/// ```
/// # use std::path::{Path, PathBuf};
///
/// use standard_traits::Get;
///
/// fn get_project_root(path: &Path) -> Option<PathBuf> {
///     todo!()
/// }
///
/// fn format_project(path: &Path) {
///     let project_root = get_project_root(path);
///     todo!()
/// }
///
/// ```
///
/// ## Implementation via `Get`
///
/// ```
/// # use std::path::{Path, PathBuf};
/// # use standard_traits::{Get, Provide};
///
/// struct ProjectRoot(PathBuf);
///
/// // note that the `Output` is `Option<ProjectRoot>` (not just `ProjectRoot`)
/// impl Provide<ProjectRoot> for &Path {
///     type Output = Option<ProjectRoot>;
///
///     fn provide(self) -> Self::Output {
///         todo!()
///     }
/// }
///
/// fn format_project(path: &Path) {
///     let project_root = path.get::<ProjectRoot>();
/// }
/// ```
pub trait Get {
    fn get<T>(self) -> <Self as Provide<T>>::Output
    where
        Self: Provide<T>;
}

impl<S> Get for S {
    fn get<T>(self) -> <Self as Provide<T>>::Output
    where
        Self: Provide<T>,
    {
        self.provide()
    }
}
