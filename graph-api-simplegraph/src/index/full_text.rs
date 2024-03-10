use fastbloom::BloomFilter;
use rphonetic::{Encoder, Metaphone};
use std::collections::HashMap;
use std::hash::Hash;

/// A phonetic full-text search index that uses Bloom filters for efficient text matching
///
/// This index provides fuzzy text search capabilities by:
/// - Using metaphone phonetic encoding to match similar-sounding words
/// - Employing Bloom filters to efficiently store and query word sets
/// - Supporting multi-word searches with "AND" semantics
///
/// # Type Parameters
/// - `V`: The type of values/keys to associate with indexed text. Must be `Eq`, `Hash` and `Copy`
#[derive(Default)]
pub struct FullTextIndex<V>
where
    V: Eq + Hash + Copy,
{
    filters: HashMap<V, BloomFilter>,
    metaphone: Metaphone,
}

impl<V> FullTextIndex<V>
where
    V: Eq + Hash + Copy,
{
    /// Indexes the given text and associates it with the specified key
    ///
    /// The text is split into words, each word is phonetically encoded,
    /// and stored in a Bloom filter optimized for the text length.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this text
    /// * `text` - The text to index
    pub(crate) fn insert(&mut self, key: V, text: &str) {
        let word_count = text.split_whitespace().count();
        let mut filter = BloomFilter::with_false_pos(0.001).expected_items(word_count);

        for word in text.split_whitespace() {
            let encoded = self.metaphone.encode(word);
            filter.insert(&encoded);
        }

        self.filters.insert(key, filter);
    }

    /// Searches the index for entries matching the given search text
    ///
    /// The search uses AND semantics - all words in the search text must match
    /// for an entry to be returned. Matching is phonetic, so similar-sounding
    /// words will match (e.g., "phone" matches "fone").
    ///
    /// # Arguments
    /// * `search` - The text to search for
    ///
    /// # Returns
    /// An iterator over matching keys
    pub(crate) fn search<'a>(&'a self, search: &str) -> impl Iterator<Item = V> + 'a {
        let search = search
            .split_whitespace()
            .map(|w| self.metaphone.encode(w))
            .collect::<Vec<_>>();
        self.filters.iter().filter_map(move |(v, filter)| {
            for word in &search {
                if !filter.contains(&word) {
                    return None;
                }
            }
            Some(*v)
        })
    }

    /// Removes an entry from the index
    ///
    /// # Arguments
    /// * `key` - The key of the entry to remove
    pub(crate) fn remove(&mut self, key: &V) {
        self.filters.remove(key);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_insert_and_search() {
        let mut index = FullTextIndex::default();
        index.insert(1, "hello world");

        let results: Vec<_> = index.search("hello").collect();
        assert_eq!(results, vec![1]);

        let results: Vec<_> = index.search("world").collect();
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn test_phonetic_matching() {
        let mut index = FullTextIndex::default();
        index.insert(1, "phone");

        let results: Vec<_> = index.search("fone").collect();
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn test_multiple_entries() {
        let mut index = FullTextIndex::default();
        index.insert(1, "hello world");
        index.insert(2, "hello there");
        index.insert(3, "goodbye world");

        let results: Vec<_> = index.search("hello").collect();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
    }

    #[test]
    fn test_remove() {
        let mut index = FullTextIndex::default();
        index.insert(1, "hello world");
        index.remove(&1);

        let results: Vec<_> = index.search("hello").collect();
        assert!(results.is_empty());
    }

    #[test]
    fn test_multi_word_search() {
        let mut index = FullTextIndex::default();
        index.insert(1, "hello beautiful world");
        index.insert(2, "hello ugly world");

        let results: Vec<_> = index.search("hello world").collect();
        assert_eq!(results.len(), 2);

        let results: Vec<_> = index.search("beautiful world").collect();
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn test_empty_cases() {
        let mut index = FullTextIndex::<i32>::default();
        let results: Vec<_> = index.search("anything").collect();
        assert!(results.is_empty());

        index.insert(1, "");
        let results: Vec<_> = index.search("").collect();
        assert_eq!(results, vec![1]);
    }
}
