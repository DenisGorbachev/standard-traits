#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::hash::{BuildHasher, Hash};

/// See also: [`TryInsert`](crate::TryInsert)
pub trait Insert<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
}

#[cfg(feature = "std")]
impl<K, V, S> Insert<K, V> for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        HashMap::insert(self, key, value)
    }
}

#[cfg(all(feature = "indexmap_2", feature = "std"))]
impl<K, V, S> Insert<K, V> for indexmap_2::IndexMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        indexmap_2::IndexMap::insert(self, key, value)
    }
}
