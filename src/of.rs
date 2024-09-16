use crate::Provide;

/// `Of` is an experimental trait analogous to [`Get`].
///
/// Compare:
///
/// * `let project_root = path.get::<ProjectRoot>()`
/// * `let project_root = ProjectRoot::of(path)`
///
/// The first example follows the regular call notation (`subject.method()`), but it's more noisy. The second example is less noisy, but uses an inverted call notation (`Trait::method(subject)`).
pub trait Of<Src> {
    fn of(source: Src) -> <Src as Provide<Self>>::Output
    where
        Src: Provide<Self>;
}

impl<T, Src: Provide<T>> Of<Src> for T {
    fn of(source: Src) -> <Src as Provide<Self>>::Output {
        source.provide()
    }
}
