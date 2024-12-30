pub trait Join<Rhs = Self> {
    type Output;

    fn join(self, rhs: Rhs) -> Self::Output;
}

#[cfg(feature = "std")]
mod impl_std {
    use super::*;
    use std::path::{Path, PathBuf};

    impl<'a> Join<&'a Path> for &Path {
        type Output = PathBuf;

        fn join(self, rhs: &'a Path) -> Self::Output {
            Path::join(self, rhs)
        }
    }

    impl<'a> Join<&'a str> for &Path {
        type Output = PathBuf;

        fn join(self, rhs: &'a str) -> Self::Output {
            Path::join(self, rhs)
        }
    }
}

#[cfg(feature = "camino_1")]
mod impl_camino_1 {
    use super::*;
    use camino_1::{Utf8Path, Utf8PathBuf};

    impl<'a> Join<&'a Utf8Path> for &Utf8Path {
        type Output = Utf8PathBuf;

        fn join(self, rhs: &'a Utf8Path) -> Self::Output {
            Utf8Path::join(self, rhs)
        }
    }

    impl<'a> Join<&'a str> for &Utf8Path {
        type Output = Utf8PathBuf;

        fn join(self, rhs: &'a str) -> Self::Output {
            Utf8Path::join(self, rhs)
        }
    }
}

#[cfg(feature = "tempfile_3")]
mod impl_tempfile_3 {
    use super::*;
    use std::path::Path;
    use tempfile_3::TempDir;

    impl<'a, 'b> Join<&'a Path> for &'b TempDir {
        type Output = <&'b Path as Join<&'a Path>>::Output;

        fn join(self, rhs: &'a Path) -> Self::Output {
            self.path().join(rhs)
        }
    }

    impl<'a, 'b> Join<&'a str> for &'b TempDir {
        type Output = <&'b Path as Join<&'a str>>::Output;

        fn join(self, rhs: &'a str) -> Self::Output {
            self.path().join(rhs)
        }
    }
}
