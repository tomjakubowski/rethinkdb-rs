use serialize::json::{Json, ToJson};
use std::ops;
use query::term_type as ty;

// NOTE: Is the phantom type actually useful/helpful outside of toy examples?
#[deriving(Clone)]
pub struct Datum<T>(Json);

impl<T> ToJson for Datum<T> {
    fn to_json(&self) -> Json {
        let Datum(ref j) = *self;
        j.clone()
    }
}

/// The ReQL NUMBER type.
#[deriving(Clone)]
pub struct Num;

/// The ReQL STRING type.
#[deriving(Clone)]
pub struct String;

pub trait ToDatum<T> for Sized? {
    fn to_datum(&self) -> Datum<T>;
}

macro_rules! impl_to_datum {
    ($ty:ty -> $rty:ty) => {
        impl ToDatum<$rty> for $ty {
            fn to_datum(&self) -> Datum<$rty> {
                Datum(self.to_json())
            }
        }
    };
}

impl_to_datum! { u8 -> Num }
impl_to_datum! { i8 -> Num }
impl_to_datum! { u16 -> Num }
impl_to_datum! { i16 -> Num }
impl_to_datum! { u32 -> Num }
impl_to_datum! { i32 -> Num }
impl_to_datum! { u64 -> Num }
impl_to_datum! { i64 -> Num }
impl_to_datum! { f32 -> Num }
impl_to_datum! { f64 -> Num }

impl<'a> ToDatum<String> for &'a str {
    fn to_datum(&self) -> Datum<String> { Datum(self.to_json()) }
}

pub fn expr<D: ToDatum<T>, T>(x: D) -> Datum<T> {
    Datum(x.to_datum().to_json())
}

// We would impl Query for Datum here but Datum queries are weird and not like
// other queries e.g. for `r::expr(420i32)` the term is just `420` on its own,
// not something like `[1, [420]]`, so the Query trait won't work. In practice
// this probably isn't a huge deal. Who wants to use RethinkDB as the identity
// function anyway?

query! {
    enum Add -> Json {
        Num { lhs: Datum<Num>, rhs: Datum<Num> },
        String { lhs: Datum<String>, rhs: Datum<String> }
    } ty::ADD
}

// Several of the remaining query terms aren't polymorphic over ReQL types so
// the enum might seem useless; purely for simplifying impl_datum_binop
query! {
    enum Sub -> Json {
        Num { lhs: Datum<Num>, rhs: Datum<Num> }
    } ty::SUB
}

query! {
    enum Mul -> Json {
        Num { lhs: Datum<Num>, rhs: Datum<Num> }
    } ty::MUL
}

query! {
    enum Div -> Json {
        Num { lhs: Datum<Num>, rhs: Datum<Num> }
    } ty::DIV
}

query! {
    enum Rem -> Json {
        Num { lhs: Datum<Num>, rhs: Datum<Num> }
    } ty::MOD
}

macro_rules! impl_datum_binop {
    ($op:ident / $fun:ident: $($ty:ident)|+) => {
        $(impl<T: ToDatum<$ty>> ops::$op<T, $op> for Datum<$ty> {
            fn $fun(&self, rhs: &T) -> $op {
                $op::$ty {
                    lhs: self.clone(),
                    rhs: rhs.to_datum()
                }
            }
        })+
    };
}

// FIXME: should Add support `Array`s?
impl_datum_binop! { Add / add: Num | String }
impl_datum_binop! { Sub / sub: Num }
impl_datum_binop! { Mul / mul: Num }
impl_datum_binop! { Div / div: Num }
impl_datum_binop! { Rem / rem: Num }

#[cfg(test)]
mod test {
    use query::expr as r;
    use serialize::json::ToJson;

    #[test]
    fn test_expr() {
        assert_eq!((r::expr(1i32)).to_json(), json!(1));
        assert_eq!((r::expr(1f64)).to_json(), json!(1.0));
        assert_eq!((r::expr("foo")).to_json(), json!("foo"));
    }

    #[test]
    fn test_ops() {
        assert_eq!((r::expr(420i32) + 123i32).to_json(), json!([24, [420, 123]]));
        assert_eq!((r::expr("foo") + "bar").to_json(), json!([24, ["foo", "bar"]]));
        assert_eq!((r::expr(420i32) - 123i32).to_json(), json!([25, [420, 123]]));
        assert_eq!((r::expr(420i32) * 123i32).to_json(), json!([26, [420, 123]]));
        assert_eq!((r::expr(420i32) / 123i32).to_json(), json!([27, [420, 123]]));
    }
}
