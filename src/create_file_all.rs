pub trait CreateFileAll {
    type Output;

    /// Create a file at `self` if it doesn't exist
    /// Also create all parent directories
    fn create_file_all(&self) -> Self::Output;
}

#[cfg(feature = "std")]
mod impl_std {
    use super::*;
    use std::fs::{create_dir_all, File, OpenOptions};
    use std::io;
    use std::path::Path;

    impl CreateFileAll for Path {
        type Output = io::Result<File>;

        fn create_file_all(&self) -> io::Result<File> {
            if self.exists() {
                OpenOptions::new().append(true).open(self)
            } else {
                if let Some(parent) = self.parent() {
                    create_dir_all(parent)?;
                }
                File::create(self)
            }
        }
    }
}

#[cfg(feature = "tempfile_3")]
mod impl_tempfile_3 {
    use super::*;
    use std::path::Path;
    use tempfile_3::TempDir;

    impl CreateFileAll for TempDir {
        type Output = <Path as CreateFileAll>::Output;

        fn create_file_all(&self) -> Self::Output {
            self.path().create_file_all()
        }
    }
}
