use std::borrow::Borrow;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::RangeBounds;

/// This ordered index allows range searches for keys.
#[derive(Debug, Default)]
pub(crate) struct OrderedIndex<K, V>
where
    K: Ord + Clone,
    V: Hash + Eq,
{
    map: BTreeMap<K, HashSet<V>>,
    empty: HashSet<V>,
}

impl<K, V> OrderedIndex<K, V>
where
    K: Eq + Hash + Clone + Debug + Ord,
    V: Eq + Hash + Clone + Copy + Debug,
{
    pub(crate) fn insert(&mut self, key: K, value: V) -> bool {
        self.map.entry(key).or_default().insert(value)
    }

    pub(crate) fn remove<Q>(&mut self, key: &Q, value: &V) -> bool
    where
        K: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
    {
        if let Some(values) = self.map.get_mut(key) {
            let removed = values.remove(value);
            if values.is_empty() {
                self.map.remove(key);
            }
            return removed;
        }
        false
    }

    pub(crate) fn get<'a, Q>(&'a self, key: &Q) -> impl Iterator<Item = V> + 'a
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.map.get(key).unwrap_or(&self.empty).iter().copied()
    }

    pub(crate) fn range<T, R>(&self, range: R) -> impl Iterator<Item = V> + '_
    where
        T: ?Sized + Ord,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        self.map.range(range).flat_map(|(_, v)| v.iter()).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_index() {
        let mut index = OrderedIndex::default();

        // Test insertion
        assert!(index.insert(1, "a"));
        assert!(index.insert(1, "b"));
        assert!(index.insert(2, "c"));

        // Test retrieval
        assert_eq!(index.get(&1).count(), 2);
        assert_eq!(index.get(&2).count(), 1);

        // Test range query
        let range_results: Vec<_> = index.range(1..3).collect();
        assert_eq!(range_results.len(), 3);

        // Test removal
        assert!(index.remove(&1, &"a"));
        assert_eq!(index.get(&1).count(), 1);

        // Test complete removal of key when no values remain
        assert!(index.remove(&1, &"b"));
        assert_eq!(index.get(&1).count(), 0);
    }

    #[test]
    fn test_duplicate_prevention() {
        let mut index = OrderedIndex::default();

        // First insertion should succeed
        assert!(index.insert(1, "a"));

        // Second insertion of same key-value should fail
        assert!(!index.insert(1, "a"));

        // Verify only one value exists
        assert_eq!(index.get(&1).count(), 1);

        // Different value with same key should succeed
        assert!(index.insert(1, "b"));
        assert_eq!(index.get(&1).count(), 2);
    }
}
