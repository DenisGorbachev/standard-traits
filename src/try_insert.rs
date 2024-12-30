pub trait TryInsert<K, V> {
    type Error<'a>
    where
        Self: 'a;

    fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, Self::Error<'_>>;
}

#[cfg(feature = "std")]
mod impl_std {
    use super::*;
    use core::hash::{BuildHasher, Hash};
    use std::collections::{
        hash_map::{Entry, OccupiedEntry},
        HashMap,
    };

    // Note: There is HashMap::try_insert in std, but it's gated under `map_try_insert`
    impl<K, V, S> TryInsert<K, V> for HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        type Error<'a>
            = OccupiedError<'a, K, V>
        where
            Self: 'a;

        fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, OccupiedError<'_, K, V>> {
            match self.entry(key) {
                Entry::Occupied(entry) => Err(OccupiedError {
                    entry,
                    value,
                }),
                Entry::Vacant(entry) => Ok(entry.insert(value)),
            }
        }
    }

    pub struct OccupiedError<'a, K: 'a, V: 'a> {
        /// The entry in the map that was already occupied.
        pub entry: OccupiedEntry<'a, K, V>,
        /// The value which was not inserted, because the entry was already occupied.
        pub value: V,
    }
}

#[cfg(feature = "indexmap_2")]
mod impl_indexmap_2 {
    use super::*;
    use core::hash::{BuildHasher, Hash};
    use indexmap_2::map::{Entry, OccupiedEntry};

    impl<K, V, S> TryInsert<K, V> for indexmap_2::IndexMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        type Error<'a>
            = OccupiedError<'a, K, V>
        where
            Self: 'a;

        fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, OccupiedError<'_, K, V>> {
            match self.entry(key) {
                Entry::Occupied(entry) => Err(OccupiedError {
                    entry,
                    value,
                }),
                Entry::Vacant(entry) => Ok(entry.insert(value)),
            }
        }
    }

    pub struct OccupiedError<'a, K: 'a, V: 'a> {
        /// The entry in the map that was already occupied.
        pub entry: OccupiedEntry<'a, K, V>,
        /// The value which was not inserted, because the entry was already occupied.
        pub value: V,
    }
}
