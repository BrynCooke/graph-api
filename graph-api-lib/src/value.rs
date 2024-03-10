use std::ops::Range;
use uuid::Uuid;

#[derive(Clone)]
pub enum Value<'a> {
    USize(usize),
    U128(u128),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    I128(i128),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    F64(f64),
    F32(f32),
    Bool(bool),
    Uuid(Uuid),
    Str(&'a str),
}

#[derive(Clone)]
pub enum ValueOrRange<'a> {
    Value(Value<'a>),
    Range(Range<Value<'a>>),
}

macro_rules! value_coercion_copy {
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
value_coercion_copy!(&'a String, Str);
value_coercion_copy!(&'a str, Str);
value_coercion_copy!(u128, U128);
value_coercion_copy!(u64, U64);
value_coercion_copy!(u32, U32);
value_coercion_copy!(u16, U16);
value_coercion_copy!(u8, U8);
value_coercion_copy!(i128, I128);
value_coercion_copy!(i64, I64);
value_coercion_copy!(i32, I32);
value_coercion_copy!(i16, I16);
value_coercion_copy!(i8, I8);
value_coercion_copy!(f64, F64);
value_coercion_copy!(f32, F32);
value_coercion_copy!(bool, Bool);
value_coercion_copy!(Uuid, Uuid);

macro_rules! value_or_range_coercion {
    ($ty: ty) => {
        impl<'a> From<$ty> for ValueOrRange<'a> {
            fn from(value: $ty) -> Self {
                Self::Value(value.into())
            }
        }

        impl<'a> From<Range<$ty>> for ValueOrRange<'a> {
            fn from(range: Range<$ty>) -> Self {
                Self::Range(range.start.into()..range.end.into())
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
