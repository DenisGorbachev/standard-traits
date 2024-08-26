#[cfg(feature = "std")]
pub trait TryInsert<K, V> {
    type Error<'a>
    where
        Self: 'a;

    fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, Self::Error<'_>>;
}

#[cfg(feature = "std")]
pub mod hash_map_impl {
    use super::*;
    use std::collections::{
        hash_map::{Entry, OccupiedEntry},
        HashMap,
    };
    use std::hash::{BuildHasher, Hash};

    // Note: There is HashMap::try_insert in std, but it's gated under `map_try_insert`
    impl<K, V, S> TryInsert<K, V> for HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        type Error<'a> = OccupiedError<'a, K, V>
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

#[cfg(all(feature = "indexmap_2", feature = "std"))]
pub mod indexmap_2_impl {
    use super::*;
    use indexmap_2::map::{Entry, OccupiedEntry};
    use std::hash::{BuildHasher, Hash};

    impl<K, V, S> TryInsert<K, V> for indexmap_2::IndexMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        type Error<'a> = OccupiedError<'a, K, V>
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
