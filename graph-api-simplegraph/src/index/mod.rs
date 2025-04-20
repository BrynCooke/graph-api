use crate::VertexId;
use graph_api_lib::{Index, IndexType, Value};
use paste::paste;
use smallbox::SmallBox;
use smallbox::space::S8;
use std::any::TypeId;
use std::ops::Deref;
use std::ops::Range;
use uuid::Uuid;

mod full_text;
mod hash;
mod range;
use std::ops::Bound;

#[derive(Debug)]
pub(crate) enum VertexIndexStorage {
    FullTextString(full_text::FullTextIndex<u32>),
    HashUuid(hash::HashIndex<Uuid, u32>),
    HashString(hash::HashIndex<String, u32>),
    HashUSize(hash::HashIndex<usize, u32>),
    HashU128(hash::HashIndex<u128, u32>),
    HashU64(hash::HashIndex<u64, u32>),
    HashU32(hash::HashIndex<u32, u32>),
    HashU16(hash::HashIndex<u16, u32>),
    HashU8(hash::HashIndex<u8, u32>),
    HashI128(hash::HashIndex<i128, u32>),
    HashI64(hash::HashIndex<i64, u32>),
    HashI32(hash::HashIndex<i32, u32>),
    HashI16(hash::HashIndex<i16, u32>),
    HashI8(hash::HashIndex<i8, u32>),
    HashBool(hash::HashIndex<bool, u32>),
    RangeUuid(range::RangeIndex<Uuid, u32>),
    RangeString(range::RangeIndex<String, u32>),
    RangeUSize(range::RangeIndex<usize, u32>),
    RangeU128(range::RangeIndex<u128, u32>),
    RangeU64(range::RangeIndex<u64, u32>),
    RangeU32(range::RangeIndex<u32, u32>),
    RangeU16(range::RangeIndex<u16, u32>),
    RangeU8(range::RangeIndex<u8, u32>),
    RangeI128(range::RangeIndex<i128, u32>),
    RangeI64(range::RangeIndex<i64, u32>),
    RangeI32(range::RangeIndex<i32, u32>),
    RangeI16(range::RangeIndex<i16, u32>),
    RangeI8(range::RangeIndex<i8, u32>),
    RangeBool(range::RangeIndex<bool, u32>),
}

impl<T: Index> From<&T> for VertexIndexStorage {
    fn from(index: &T) -> Self {
        macro_rules! index {
            ($ty:ty, $ident: ident) => {
                paste! {
                    if index.ty() == TypeId::of::<$ty>() {
                        match index.index_type() {
                            IndexType::FullText => {
                                return VertexIndexStorage::FullTextString(Default::default());
                            }
                            IndexType::Range => {
                                return VertexIndexStorage::[<Range $ident>](Default::default());
                            }
                            IndexType::Hash => {
                                return VertexIndexStorage::[<Hash $ident>](Default::default());
                            },
                            _=>{}
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
            ($ty: ident, $index: ident, $conversion: expr) => {
                paste! {
                    match (&key, index.index_type()) {
                        (Value::Str(key), IndexType::FullText) => {
                            if let VertexIndexStorage::FullTextString(index) = self {
                                index.insert(value, key);
                                return
                            }
                        },
                        (Value::$ty(key), IndexType::Range) => {
                            if let VertexIndexStorage::[<Range $index>](index) = self {
                                index.insert((*key).$conversion(), value);
                                return
                            }
                        },
                        (Value::$ty(key), IndexType::Hash) => {
                            if let VertexIndexStorage::[<Hash $index>](index) = self {
                                index.insert((*key).$conversion(), value);
                                return
                            }
                        },
                        _=>{}
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
        panic!("unsupported index type {:?}({})", index, index.index_type())
    }
    pub(crate) fn remove<I: Index>(&mut self, key: &Value, value: u32, index: &I) {
        macro_rules! remove {
            ($ty: ident) => {
                remove!($ty, $ty)
            };
            ($ty: ident, $index: ident) => {
                paste! {
                    if let Value::$ty(key) = key {
                        let key = key.deref();
                        match index.index_type() {
                            IndexType::FullText => {
                                if let VertexIndexStorage::FullTextString(index) = self {
                                    index.remove(&value);
                                }
                            },
                            IndexType::Range => {
                                if let VertexIndexStorage::[<Range $index>](index) = self {
                                    index.remove(key, &value);
                                }
                            },
                            IndexType::Hash => {
                                if let VertexIndexStorage::[<Hash $index>](index) = self {
                                    index.remove(key, &value);
                                }
                            }
                            _=>{}
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
            ($ty: ident, $index: ident) => {
                paste! {
                    match (key, index.index_type()) {
                        (Value::Str(key), IndexType::FullText) => {
                            let key = key.deref();
                            if let VertexIndexStorage::FullTextString(index) = self {
                                return smallbox::smallbox!(index.search(key).map(move |id| VertexId::new(label, id)));
                            }
                        },
                        (Value::$ty(key), IndexType::Range) => {
                            let key = key.deref();
                            if let VertexIndexStorage::[<Range $index>](index) = self {
                                return smallbox::smallbox!(index.get(key).map(move |id| VertexId::new(label, id)));
                            }
                        },
                        (Value::$ty(key), IndexType::Hash) => {
                            let key = key.deref();
                            if let VertexIndexStorage::[<Hash $index>](index) = self {
                                return smallbox::smallbox!(index.get(key).map(move |id| VertexId::new(label, id)));
                            }
                        },
                        _=>{}
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
                        if let VertexIndexStorage::[<Range $index>](index) = self {
                            return smallbox::smallbox!(index.range::<$ty,_ >((Bound::Included(*start), Bound::Excluded(*end))).map(move |id| VertexId::new(label, id)));
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
