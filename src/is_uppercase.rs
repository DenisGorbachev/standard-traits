use alloc::string::String;

pub trait IsUppercase {
    fn is_uppercase(&self) -> bool;
}

impl IsUppercase for char {
    fn is_uppercase(&self) -> bool {
        char::is_uppercase(*self)
    }
}

impl IsUppercase for str {
    fn is_uppercase(&self) -> bool {
        self.chars().all(char::is_uppercase)
    }
}

impl IsUppercase for String {
    fn is_uppercase(&self) -> bool {
        self.chars().all(char::is_uppercase)
    }
}
