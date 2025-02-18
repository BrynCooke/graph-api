use crate::VertexId;
use graph_api_lib::{Index, Value};
use paste::paste;
use smallbox::space::S8;
use smallbox::SmallBox;
use std::any::TypeId;
use std::ops::Deref;
use std::ops::Range;
use uuid::Uuid;

mod full_text;
mod ordered;
mod unordered;
use std::ops::Bound;

pub(crate) enum VertexIndexStorage {
    FullTextString(full_text::FullTextIndex<u32>),
    UnorderedUuid(unordered::UnorderedIndex<Uuid, u32>),
    UnorderedString(unordered::UnorderedIndex<String, u32>),
    UnorderedUSize(unordered::UnorderedIndex<usize, u32>),
    UnorderedU128(unordered::UnorderedIndex<u128, u32>),
    UnorderedU64(unordered::UnorderedIndex<u64, u32>),
    UnorderedU32(unordered::UnorderedIndex<u32, u32>),
    UnorderedU16(unordered::UnorderedIndex<u16, u32>),
    UnorderedU8(unordered::UnorderedIndex<u8, u32>),
    UnorderedI128(unordered::UnorderedIndex<i128, u32>),
    UnorderedI64(unordered::UnorderedIndex<i64, u32>),
    UnorderedI32(unordered::UnorderedIndex<i32, u32>),
    UnorderedI16(unordered::UnorderedIndex<i16, u32>),
    UnorderedI8(unordered::UnorderedIndex<i8, u32>),
    UnorderedBool(unordered::UnorderedIndex<bool, u32>),
    OrderedUuid(ordered::OrderedIndex<Uuid, u32>),
    OrderedString(ordered::OrderedIndex<String, u32>),
    OrderedUSize(ordered::OrderedIndex<usize, u32>),
    OrderedU128(ordered::OrderedIndex<u128, u32>),
    OrderedU64(ordered::OrderedIndex<u64, u32>),
    OrderedU32(ordered::OrderedIndex<u32, u32>),
    OrderedU16(ordered::OrderedIndex<u16, u32>),
    OrderedU8(ordered::OrderedIndex<u8, u32>),
    OrderedI128(ordered::OrderedIndex<i128, u32>),
    OrderedI64(ordered::OrderedIndex<i64, u32>),
    OrderedI32(ordered::OrderedIndex<i32, u32>),
    OrderedI16(ordered::OrderedIndex<i16, u32>),
    OrderedI8(ordered::OrderedIndex<i8, u32>),
    OrderedBool(ordered::OrderedIndex<bool, u32>),
}

impl<T: Index> From<&T> for VertexIndexStorage {
    fn from(index: &T) -> Self {
        macro_rules! index {
            ($ty:ty, $ident: ident) => {
                paste! {
                    if index.ty() == TypeId::of::<$ty>() {
                        if index.full_text() {
                            return VertexIndexStorage::FullTextString(Default::default())
                        }
                        if index.ordered() {
                            return VertexIndexStorage::[<Ordered $ident>](Default::default())
                        } else {
                            return VertexIndexStorage::[<Unordered $ident>](Default::default())
                        }
                    }
                }
            };
        }

        index!(String, String);
        index!(Uuid, Uuid);
        index!(usize, USize);
        index!(u128, U128);
        index!(u64, U64);
        index!(u32, U32);
        index!(u16, U16);
        index!(u8, U8);
        index!(i128, I128);
        index!(i64, I64);
        index!(i32, I32);
        index!(i16, I16);
        index!(i8, I8);
        index!(bool, Bool);
        panic!("unsupported index type {:?}", index)
    }
}

impl VertexIndexStorage {
    pub(crate) fn insert<I: Index>(&mut self, key: Value, value: u32, index: &I) {
        macro_rules! insert {
            ($ty: ident) => {
                insert!($ty, $ty, into)
            };
            ($ty: ident, $ident: ident, $conversion: expr) => {
                paste! {
                    if let Value::Str(key) = key {
                        if index.full_text() {
                            if let VertexIndexStorage::FullTextString(index) = self {
                                // full text indexes are inverted
                                index.insert(value, key);
                            }
                        }
                    }
                    if let Value::$ty(key) = key {
                        if index.ordered() {
                                if let VertexIndexStorage::[<Ordered $ident>](index) = self {
                                    index.insert(key.$conversion(), value);
                                }
                        } else {
                                if let VertexIndexStorage::[<Unordered $ident>](index) = self {
                                    index.insert(key.$conversion(), value);
                                }

                        }
                        return;
                    }
                }
            };
        }
        insert!(Str, String, to_string);
        insert!(USize);
        insert!(U128);
        insert!(U64);
        insert!(U32);
        insert!(U16);
        insert!(U8);
        insert!(I128);
        insert!(I64);
        insert!(I32);
        insert!(I16);
        insert!(I8);
        insert!(Bool);
        insert!(Uuid);
        panic!("unsupported index type {:?}", index)
    }
    pub(crate) fn remove<I: Index>(&mut self, key: &Value, value: u32, index: &I) {
        macro_rules! remove {
            ($ty: ident) => {
                remove!($ty, $ty)
            };
            ($ty: ident, $ident: ident) => {
                paste! {
                    if let Value::$ty(key) = key {
                        let key = key.deref();
                        if index.full_text() {
                            if let VertexIndexStorage::FullTextString(index) = self {
                                index.remove(&value);
                            }
                        }
                        else if index.ordered() {
                            if let VertexIndexStorage::[<Ordered $ident>](index) = self {
                                index.remove(key, &value);
                            }
                        } else {
                            if let VertexIndexStorage::[<Unordered $ident>](index) = self {
                                index.remove(key, &value);
                            }
                        }
                        return;
                    }
                }
            };
        }

        remove!(Str, String);
        remove!(USize);
        remove!(U128);
        remove!(U64);
        remove!(U32);
        remove!(U16);
        remove!(U8);
        remove!(I128);
        remove!(I64);
        remove!(I32);
        remove!(I16);
        remove!(I8);
        remove!(Bool);
        remove!(Uuid);
        panic!("unsupported index type {:?}", index)
    }

    pub(crate) fn get<'a, I: Index>(
        &'a self,
        key: &Value,
        index: &I,
    ) -> SmallBox<dyn Iterator<Item = VertexId> + 'a, S8> {
        let label = 0;
        macro_rules! search {
            ($ident: ident) => {
                search!($ident, $ident);
            };
            ($ident: ident, $index: ident) => {
                paste! {
                    if let Value::Str(key) = key {
                        if index.full_text() {
                            if let VertexIndexStorage::FullTextString(index) = self {
                                return smallbox::smallbox!(index.search(key).map(move |id| VertexId::new(label, id)));
                            }
                        }
                    }
                    if let Value::$ident(key) = key {
                        let key = key.deref();
                        if index.ordered() {
                            if let VertexIndexStorage::[<Ordered $index>](index) = self {
                                return smallbox::smallbox!(index.get(key).map(move |id| VertexId::new(label, id)));
                            }
                        } else {
                            if let VertexIndexStorage::[<Unordered $index>](index) = self {
                                return smallbox::smallbox!(index.get(key).map(move |id| VertexId::new(label, id)));
                            }
                        }
                    }
                }
            };
        }
        search!(Str, String);
        search!(USize);
        search!(U128);
        search!(U64);
        search!(U32);
        search!(U16);
        search!(U8);
        search!(I128);
        search!(I64);
        search!(I32);
        search!(I16);
        search!(I8);
        search!(Bool);
        search!(Uuid);
        panic!("unsupported index type {:?}", index)
    }

    pub(crate) fn range<'a, I: Index>(
        &'a self,
        range: &Range<Value>,
        index: &I,
    ) -> SmallBox<dyn Iterator<Item = VertexId> + 'a, S8> {
        let label = 0;
        macro_rules! search {
            ($ident: ident) => {
                paste! {
                    search!($ident, [<$ident:lower>], $ident);
                }
            };
            ($ident: ident, $ty: ty) => {
                search!($ident, $ty, $ident);
            };
            ($ident: ident, $ty: ty, $index: ident) => {
                paste! {
                    if let (Value::$ident(start), Value::$ident(end)) = (&range.start, &range.end) {
                        if index.ordered() {
                            if let VertexIndexStorage::[<Ordered $index>](index) = self {
                                return smallbox::smallbox!(index.range::<$ty,_ >((Bound::Included(*start), Bound::Excluded(*end))).map(move |id| VertexId::new(label, id)));
                            }
                        } else {
                            panic!("unordered index does not support range search");
                        }
                    }
                }
            };
        }
        search!(Str, str, String);
        search!(USize);
        search!(U128);
        search!(U64);
        search!(U32);
        search!(U16);
        search!(U8);
        search!(I128);
        search!(I64);
        search!(I32);
        search!(I16);
        search!(I8);
        search!(Bool);
        panic!("unsupported index type {:?}", index)
    }
}
