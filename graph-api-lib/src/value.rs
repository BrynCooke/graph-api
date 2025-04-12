use std::ops::Range;
use uuid::Uuid;

/// A polymorphic value type for graph element properties.
///
/// This enum represents the various types of values that can be stored in graph
/// vertices and edges. It supports numeric types, boolean values, UUIDs, and strings.
///
/// The lifetime parameter allows the enum to store borrowed string data.
#[derive(Clone)]
pub enum Value<'a> {
    /// An unsigned size value
    USize(usize),
    /// A 128-bit unsigned integer
    U128(u128),
    /// A 64-bit unsigned integer
    U64(u64),
    /// A 32-bit unsigned integer
    U32(u32),
    /// A 16-bit unsigned integer
    U16(u16),
    /// An 8-bit unsigned integer
    U8(u8),
    /// A 128-bit signed integer
    I128(i128),
    /// A 64-bit signed integer
    I64(i64),
    /// A 32-bit signed integer
    I32(i32),
    /// A 16-bit signed integer
    I16(i16),
    /// An 8-bit signed integer
    I8(i8),
    /// A 64-bit floating point number
    F64(f64),
    /// A 32-bit floating point number
    F32(f32),
    /// A boolean value
    Bool(bool),
    /// A UUID
    Uuid(Uuid),
    /// A string slice
    Str(&'a str),
}

/// A range of Values for range-based queries and indexes.
///
/// This struct wraps a standard Rust Range of Value types, allowing for
/// range-based lookups and queries in indexed graphs.
///
/// The lifetime parameter corresponds to the lifetime of any string data in the range.
#[derive(Clone)]
pub struct ValueRange<'a>(pub(crate) Range<Value<'a>>);

macro_rules! value_coercion {
    ($ty: ty,  $ident: ident) => {
        impl<'a> From<&'a $ty> for Value<'a> {
            fn from(value: &'a $ty) -> Self {
                Self::$ident(*value)
            }
        }
        impl<'a> From<&'a mut $ty> for Value<'a> {
            fn from(value: &'a mut $ty) -> Self {
                Self::$ident(*value)
            }
        }
        impl<'a> From<$ty> for Value<'a> {
            fn from(value: $ty) -> Self {
                Self::$ident(value)
            }
        }
    };
}
value_coercion!(&'a String, Str);
value_coercion!(&'a str, Str);
value_coercion!(u128, U128);
value_coercion!(u64, U64);
value_coercion!(u32, U32);
value_coercion!(u16, U16);
value_coercion!(u8, U8);
value_coercion!(i128, I128);
value_coercion!(i64, I64);
value_coercion!(i32, I32);
value_coercion!(i16, I16);
value_coercion!(i8, I8);
value_coercion!(f64, F64);
value_coercion!(f32, F32);
value_coercion!(bool, Bool);
value_coercion!(Uuid, Uuid);

macro_rules! value_or_range_coercion {
    ($ty: ty) => {
        impl<'a> From<Range<$ty>> for ValueRange<'a> {
            fn from(range: Range<$ty>) -> Self {
                ValueRange(range.start.into()..range.end.into())
            }
        }
    };
}

value_or_range_coercion!(&'a str);
value_or_range_coercion!(u128);
value_or_range_coercion!(u64);
value_or_range_coercion!(u32);
value_or_range_coercion!(u16);
value_or_range_coercion!(u8);
value_or_range_coercion!(i128);
value_or_range_coercion!(i64);
value_or_range_coercion!(i32);
value_or_range_coercion!(i16);
value_or_range_coercion!(i8);
value_or_range_coercion!(f64);
value_or_range_coercion!(f32);
value_or_range_coercion!(bool);
value_or_range_coercion!(Uuid);
value_or_range_coercion!(&'a String);
