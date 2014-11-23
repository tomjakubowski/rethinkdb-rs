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

#[deriving(Clone)]
pub struct Num;
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

// We could impl Query here but Datum queries are weird and non-standard e.g.
// for r::expr(420) the term is supposed to be just `1` on its own, not
// something like [1, [420]], so the Query trait won't work without making the
// common case annoying/wasteful (i.e. by making args() return Option<Vec>)
// In practice this probably isn't a huge deal. Who wants to use RethinkDB as
// the identity function anyway?

query! {
    enum Add -> Json { // NOTE: we know it's Number -> Number or String -> String, so...
        Num { lhs: Datum<Num>, rhs: Datum<Num> },
        String { lhs: Datum<String>, rhs: Datum<String> }
    } ty::ADD
}

impl<T: ToDatum<Num>> ops::Add<T, Add> for Datum<Num> {
    fn add(&self, rhs: &T) -> Add {
        Add::Num {
            lhs: self.clone(),
            rhs: rhs.to_datum()
        }
    }
}

impl<T: ToDatum<String>> ops::Add<T, Add> for Datum<String> {
    fn add(&self, rhs: &T) -> Add {
        Add::String {
            lhs: self.clone(),
            rhs: rhs.to_datum()
        }
    }
}

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
    }
}
