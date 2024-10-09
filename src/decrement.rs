use core::ops::SubAssign;

pub trait Decrement {
    fn decrement(self);
}

impl Decrement for &mut u32 {
    fn decrement(self) {
        self.sub_assign(1)
    }
}

impl Decrement for &mut u64 {
    fn decrement(self) {
        self.sub_assign(1)
    }
}

impl Decrement for &mut usize {
    fn decrement(self) {
        self.sub_assign(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Decrement;

    #[test]
    fn must_decrement_u32() {
        let mut ten: u32 = 10;
        ten.decrement();
        assert_eq!(ten, 9);
    }
}
