/// A vector implementation that uses tombstone markers for efficient deletions
///
/// Instead of removing elements and shifting the remaining ones, this vector marks deleted elements
/// as tombstones (None). When inserting new elements, these empty slots are reused before growing
/// the underlying vector.
///
/// # Benefits
/// - O(1) deletion time complexity since no shifting is required
/// - Stable indices - once an element is inserted, its index never changes
/// - Memory reuse through tombstone slot recycling
///
/// # Implementation Details
/// - Uses an internal Vec<Option<T>> to store elements, where None represents a tombstone
/// - Maintains a separate Vec<usize> to track available tombstone slots for reuse
/// - When inserting, checks tombstones first before pushing to the end
/// - When removing, marks the slot as None and adds its index to tombstones
///
pub(crate) struct TombstoneVec<T> {
    tombstones: Vec<usize>,
    data: Vec<Option<T>>,
}

impl<T> Default for TombstoneVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> TombstoneVec<T> {
    /// Creates a new empty TombstoneVec
    pub(crate) fn new() -> Self {
        Self {
            tombstones: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Pushes a value onto the vector, using a tombstone slot if available
    pub(crate) fn push(&mut self, value: T) -> usize {
        if let Some(index) = self.tombstones.pop() {
            self.data[index] = Some(value);
            index
        } else {
            self.data.push(Some(value));
            self.data.len() - 1
        }
    }

    /// Removes an element at the given index, marking it as a tombstone
    pub(crate) fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.data.len() {
            return None;
        }

        let value = self.data[index].take();
        if value.is_some() {
            self.tombstones.push(index);
        }
        value
    }

    /// Returns a reference to the value at the given index
    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index).and_then(|opt| opt.as_ref())
    }

    /// Returns a mutable reference to the value at the given index
    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index).and_then(|opt| opt.as_mut())
    }

    /// Returns the number of actual elements (excluding tombstones)
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.data.len() - self.tombstones.len()
    }

    /// Returns true if the vector contains no elements
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the valid elements
    pub(crate) fn index_iter(&self) -> TombstoneVecIter<'_, T> {
        TombstoneVecIter {
            data: &self.data,
            index: 0,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.data.clear();
        self.tombstones.clear();
    }
}

impl<T> std::ops::Index<usize> for TombstoneVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.data[index].as_ref().expect("expected element")
    }
}

impl<T> std::ops::IndexMut<usize> for TombstoneVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data[index].as_mut().expect("expected element")
    }
}

/// Iterator for TombstoneVec that returns indexes of valid elements
pub(crate) struct TombstoneVecIter<'a, T> {
    data: &'a [Option<T>],
    index: usize,
}

impl<T> Iterator for TombstoneVecIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.len() {
            let data = &self.data[self.index];
            self.index += 1;
            if data.is_some() {
                return Some(self.index - 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_get() {
        let mut vec = TombstoneVec::new();
        let idx1 = vec.push(1);
        let idx2 = vec.push(2);
        let idx3 = vec.push(3);

        assert_eq!(vec.get(idx1), Some(&1));
        assert_eq!(vec.get(idx2), Some(&2));
        assert_eq!(vec.get(idx3), Some(&3));
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_remove_and_reuse() {
        let mut vec = TombstoneVec::new();
        let idx1 = vec.push(1);
        let _idx2 = vec.push(2);

        assert_eq!(vec.remove(idx1), Some(1));
        assert_eq!(vec.get(idx1), None);
        assert_eq!(vec.len(), 1);

        // Reuse the tombstone slot
        let idx3 = vec.push(3);
        assert_eq!(idx3, idx1); // Should reuse the first index
        assert_eq!(vec.get(idx3), Some(&3));
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_index_iteration() {
        let mut vec = TombstoneVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.remove(1); // Remove middle element

        let collected: Vec<_> = vec.index_iter().collect();
        assert_eq!(collected, vec![0, 2]);
    }

    #[test]
    fn test_get_mut() {
        let mut vec = TombstoneVec::new();
        let idx = vec.push(1);

        if let Some(value) = vec.get_mut(idx) {
            *value = 42;
        }

        assert_eq!(vec.get(idx), Some(&42));
    }

    #[test]
    fn test_empty_operations() {
        let mut vec = TombstoneVec::<i32>::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.remove(0), None);
        assert_eq!(vec.get(0), None);
    }

    #[test]
    fn test_multiple_remove_push_cycles() {
        let mut vec = TombstoneVec::new();
        let idx1 = vec.push(1);
        let idx2 = vec.push(2);

        vec.remove(idx1);
        vec.remove(idx2);

        let new_idx1 = vec.push(3);
        let new_idx2 = vec.push(4);

        assert_eq!(new_idx1, idx2); // Should reuse second tombstone as it was pushed last
        assert_eq!(new_idx2, idx1); // Should reuse first tombstone
        assert_eq!(vec.get(new_idx1), Some(&3));
        assert_eq!(vec.get(new_idx2), Some(&4));
    }

    #[test]
    fn test_len_behavior() {
        let mut vec = TombstoneVec::new();
        assert_eq!(vec.len(), 0);

        // Adding elements increases length
        vec.push(1);
        assert_eq!(vec.len(), 1);
        vec.push(2);
        assert_eq!(vec.len(), 2);
        vec.push(3);
        assert_eq!(vec.len(), 3);

        // Removing elements decreases length
        vec.remove(1);
        assert_eq!(vec.len(), 2);

        // Reusing tombstone slot maintains length
        vec.push(4); // Reuses index 1
        assert_eq!(vec.len(), 3);

        // Multiple removals
        vec.remove(0);
        vec.remove(2);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_len_with_invalid_removes() {
        let mut vec = TombstoneVec::new();
        vec.push(1);
        vec.push(2);

        // Removing same index multiple times
        assert_eq!(vec.len(), 2);
        vec.remove(1);
        assert_eq!(vec.len(), 1);
        vec.remove(1); // Already removed
        assert_eq!(vec.len(), 1);

        // Removing out of bounds
        vec.remove(99);
        assert_eq!(vec.len(), 1);
    }
}
