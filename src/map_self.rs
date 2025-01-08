use alloc::vec::Vec;

/// This trait is used for one-shot mapping. If you need to chain multiple methods, use normal iterators.
pub trait MapSelf<Mapper> {
    type Output;

    fn map_self(self, mapper: Mapper) -> Self::Output;
}

impl<A, B, Mapper: FnMut(A) -> B> MapSelf<Mapper> for Vec<A> {
    type Output = Vec<B>;

    #[inline(always)]
    fn map_self(self, mapper: Mapper) -> Self::Output {
        self.into_iter().map(mapper).collect()
    }
}

impl<'a, A, B, Mapper: FnMut(&'a A) -> B> MapSelf<Mapper> for &'a Vec<A> {
    type Output = Vec<B>;

    #[inline(always)]
    fn map_self(self, mapper: Mapper) -> Self::Output {
        self.iter().map(mapper).collect()
    }
}

impl<A, B, Mapper: FnMut(&mut A) -> B> MapSelf<Mapper> for &mut Vec<A> {
    type Output = Vec<B>;

    #[inline(always)]
    fn map_self(self, mapper: Mapper) -> Self::Output {
        self.iter_mut().map(mapper).collect()
    }
}

impl<'a, A, B, Mapper: FnMut(&'a A) -> B> MapSelf<Mapper> for &'a [A] {
    type Output = Vec<B>;

    #[inline(always)]
    fn map_self(self, mapper: Mapper) -> Self::Output {
        self.iter().map(mapper).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn must_map_vector_own() {
        let a = vec![1, 2, 3];
        let b = a.map_self(|el| el + 1);
        // `a` has been consumed by `Map::map`
        assert_eq!(b, vec![2, 3, 4]);
    }

    #[test]
    fn must_map_vector_ref() {
        let a = vec![1, 2, 3];
        let b = (&a).map_self(|el| el + 1);
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![2, 3, 4]);
    }

    #[test]
    fn must_map_vector_mut() {
        let mut a = vec![1, 2, 3];
        let b = (&mut a).map_self(|el| *el += 1);
        assert_eq!(a, vec![2, 3, 4]);
        assert_eq!(b, vec![(), (), ()])
    }
}
