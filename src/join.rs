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

        fn join(self, rhs: &'a Path) -> PathBuf {
            Path::join(self, rhs)
        }
    }

    impl<'a> Join<&'a str> for &Path {
        type Output = PathBuf;

        fn join(self, rhs: &'a str) -> PathBuf {
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

        fn join(self, rhs: &'a Utf8Path) -> Utf8PathBuf {
            Utf8Path::join(self, rhs)
        }
    }

    impl<'a> Join<&'a str> for &Utf8Path {
        type Output = Utf8PathBuf;

        fn join(self, rhs: &'a str) -> Utf8PathBuf {
            Utf8Path::join(self, rhs)
        }
    }
}
