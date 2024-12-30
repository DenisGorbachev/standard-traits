pub trait WriteFileAll<Contents> {
    type Output;

    /// Create a file at `self` if it doesn't exist
    /// Also create all parent directories
    /// Write the `contents` to the file
    fn write_file_all(&self, contents: Contents) -> Self::Output;
}

#[cfg(feature = "std")]
mod impl_std {
    use super::*;
    use crate::CreateFileAll;
    use std::fs::File;
    use std::io;
    use std::io::Write;
    use std::path::Path;

    impl WriteFileAll<&[u8]> for Path {
        type Output = io::Result<File>;

        fn write_file_all(&self, contents: &[u8]) -> Self::Output {
            let mut file = self.create_file_all()?;
            file.write_all(contents)?;
            Ok(file)
        }
    }
}

#[cfg(feature = "tempfile_3")]
mod impl_tempfile_3 {
    use super::*;
    use std::path::Path;
    use tempfile_3::TempDir;

    impl<'a> WriteFileAll<&'a [u8]> for TempDir {
        type Output = <Path as WriteFileAll<&'a [u8]>>::Output;

        fn write_file_all(&self, contents: &'a [u8]) -> Self::Output {
            self.path().write_file_all(contents)
        }
    }
}
