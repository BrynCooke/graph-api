use crate::{Index, Value};
use std::fmt::Debug;
use std::hash::Hash;

/// An element in a graph. This is either an edge or a vertex.
pub trait Element: Debug {
    type Label: Eq + Copy + Hash + Debug + Label + 'static;
    type Index: Eq + Copy + Hash + Debug + Index + 'static;

    /// Returns the label of the element.
    fn label(&self) -> Self::Label;
    fn value(&self, _index: &Self::Index) -> Option<Value> {
        None
    }
    fn indexes() -> &'static [Self::Index] {
        &[]
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
