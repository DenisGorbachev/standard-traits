use core::ops::SubAssign;

pub trait DecrementMut {
    fn decrement_mut(&mut self);
}

impl DecrementMut for u32 {
    fn decrement_mut(&mut self) {
        self.sub_assign(1)
    }
}

impl DecrementMut for u64 {
    fn decrement_mut(&mut self) {
        self.sub_assign(1)
    }
}

impl DecrementMut for usize {
    fn decrement_mut(&mut self) {
        self.sub_assign(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::DecrementMut;

    #[test]
    fn must_decrement_u32() {
        let mut ten: u32 = 10;
        ten.decrement_mut();
        assert_eq!(ten, 9);
    }
}
