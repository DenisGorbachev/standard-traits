pub trait IsUppercase {
    fn is_uppercase(&self) -> bool;
}

impl IsUppercase for char {
    fn is_uppercase(&self) -> bool {
        char::is_uppercase(*self)
    }
}
