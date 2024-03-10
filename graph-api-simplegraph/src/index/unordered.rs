use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

/// Unordered index allows multiple values for a key
/// This is useful for things such as labels
#[derive(Debug, Default)]
pub(crate) struct UnorderedIndex<K, V> {
    map: HashMap<K, HashSet<V>>,
    empty: HashSet<V>,
}

impl<K, V> UnorderedIndex<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Eq + Hash + Clone + Copy + Debug,
{
    pub(crate) fn insert(&mut self, key: K, value: V) {
        self.map.entry(key).or_default().insert(value);
    }

    pub(crate) fn remove<Q>(&mut self, key: &Q, value: &V)
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        if let Some(values) = self.map.get_mut(key) {
            values.remove(value);
            if values.is_empty() {
                self.map.remove(key);
            }
        }
    }

    pub(crate) fn get<'a, Q>(&'a self, key: &Q) -> impl Iterator<Item = V> + 'a
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.get(key).unwrap_or(&self.empty).iter().copied()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_maintains_uniqueness() {
        let mut index = UnorderedIndex::default();

        // Insert same value multiple times
        index.insert("key1", 1);
        index.insert("key1", 1);
        index.insert("key1", 1);

        // Should only contain one instance
        let results: Vec<_> = index.get(&"key1").collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], 1);
    }

    #[test]
    fn test_multiple_keys_uniqueness() {
        let mut index = UnorderedIndex::default();

        // Insert same value under different keys
        index.insert("key1", 1);
        index.insert("key2", 1);
        index.insert("key1", 2);

        // Check key1
        let results1: Vec<_> = index.get("key1").collect();
        assert_eq!(results1.len(), 2);
        assert!(results1.contains(&1));
        assert!(results1.contains(&2));

        // Check key2
        let results2: Vec<_> = index.get("key2").collect();
        assert_eq!(results2.len(), 1);
        assert_eq!(results2[0], 1);
    }

    #[test]
    fn test_remove_maintains_uniqueness() {
        let mut index = UnorderedIndex::default();

        // Setup
        index.insert("key1", 1);
        index.insert("key1", 2);

        // Remove same value multiple times
        index.remove(&"key1", &1);
        index.remove(&"key1", &1); // Should have no effect

        let results: Vec<_> = index.get("key1").collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], 2);
    }
}
