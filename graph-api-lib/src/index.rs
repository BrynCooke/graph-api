use std::any::TypeId;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Index
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    fn variants() -> &'static [Self];

    fn ty(&self) -> TypeId {
        unimplemented!("index not implemented")
    }

    fn ordinal(&self) -> usize {
        unimplemented!("index not implemented")
    }

    fn ordered(&self) -> bool {
        false
    }

    fn full_text(&self) -> bool {
        false
    }
}

impl Index for () {
    fn variants() -> &'static [Self] {
        &[]
    }
}
