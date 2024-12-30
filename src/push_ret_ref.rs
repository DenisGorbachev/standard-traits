use alloc::vec::Vec;

pub trait PushRetRef<T> {
    fn push_ret_ref(&mut self, value: T) -> &T;
}

impl<T> PushRetRef<T> for Vec<T> {
    fn push_ret_ref(&mut self, value: T) -> &T {
        <Vec<T>>::push(self, value);
        self.last().unwrap()
    }
}
