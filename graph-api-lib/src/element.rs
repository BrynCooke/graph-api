use crate::{Label, Value};
use std::fmt::Debug;
use std::hash::Hash;

/// An element in a graph. This is either an edge or a vertex.
pub trait Element: Debug {
    /// Information about the Label for this element
    type Label: Eq + Copy + Hash + Debug + Label + 'static;

    /// Returns the label of the element.
    fn label(&self) -> Self::Label;

    /// Given an index returns the value that is associated with this index.
    fn value(&self, _index: &<Self::Label as Label>::Index) -> Option<Value> {
        None
    }
}

impl Element for () {
    type Label = ();

    fn label(&self) -> Self::Label {}
}

impl Element for u32 {
    type Label = ();

    fn label(&self) -> Self::Label {}
}

impl Element for u64 {
    type Label = ();

    fn label(&self) -> Self::Label {}
}

impl Element for f32 {
    type Label = ();

    fn label(&self) -> Self::Label {}
}

impl Element for f64 {
    type Label = ();

    fn label(&self) -> Self::Label {}
}
