use crate::{Index, Value};
use std::fmt::Debug;
use std::hash::Hash;

/// An element in a graph. This is either an edge or a vertex.
pub trait Element: Debug {
    /// Information about the Label for this element
    type Label: Eq + Copy + Hash + Debug + Label + 'static;

    /// Information about indexes for this element
    type Index: Eq + Copy + Hash + Debug + Index + 'static;

    /// Returns the label of the element.
    fn label(&self) -> Self::Label;

    /// Given an index returns the value that is associated with this index.
    fn value(&self, _index: &Self::Index) -> Option<Value> {
        None
    }
}

pub trait Label {
    const COUNT: usize;
    fn ordinal(&self) -> usize;
}

impl Element for () {
    type Label = ();
    type Index = ();

    fn label(&self) -> Self::Label {}
}

impl Element for u32 {
    type Label = ();
    type Index = ();

    fn label(&self) -> Self::Label {}
}

impl Element for u64 {
    type Label = ();
    type Index = ();

    fn label(&self) -> Self::Label {}
}

impl Element for f32 {
    type Label = ();
    type Index = ();

    fn label(&self) -> Self::Label {}
}

impl Element for f64 {
    type Label = ();
    type Index = ();

    fn label(&self) -> Self::Label {}
}

impl Label for () {
    const COUNT: usize = 1;

    fn ordinal(&self) -> usize {
        0
    }
}
