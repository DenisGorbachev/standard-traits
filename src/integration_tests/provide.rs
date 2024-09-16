#![cfg(all(test, feature = "std", feature = "unstable_get", feature = "unstable_of"))]

use crate::Get;
use crate::Of;
use crate::Provide;
use std::path::{Path, PathBuf};

pub struct ProjectRoot(pub PathBuf);

impl Provide<ProjectRoot> for &Path {
    type Output = Option<ProjectRoot>;

    fn provide(self) -> Self::Output {
        self.ancestors()
            .find(|ancestor| ancestor.join("Cargo.toml").exists())
            .map(|ancestor| ProjectRoot(ancestor.into()))
    }
}

#[test]
fn get_and_of_must_integrate_with_provide() {
    let path_buf = PathBuf::from("/projects/mesa/src/types.rs");
    let path: &Path = path_buf.as_ref();
    let _root = path.get::<ProjectRoot>();
    let _root = ProjectRoot::of(path);
}
