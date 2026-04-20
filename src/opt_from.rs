pub trait OptFrom<T>: Sized {
    fn opt_from(value: T) -> Option<Self>;
}
