use crate::Provide;

/// `Get` is an experimental trait that simplifies the use of [`Provide`].
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
