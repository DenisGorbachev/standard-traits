use core::ops::SubAssign;

pub trait DecrementMut {
    fn decrement(&mut self);
}

impl DecrementMut for u32 {
    fn decrement(&mut self) {
        self.sub_assign(1)
    }
}

impl DecrementMut for u64 {
    fn decrement(&mut self) {
        self.sub_assign(1)
    }
}

impl DecrementMut for usize {
    fn decrement(&mut self) {
        self.sub_assign(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::DecrementMut;

    #[test]
    fn must_decrement_u32() {
        let mut ten: u32 = 10;
        ten.decrement();
        assert_eq!(ten, 9);
    }
}
