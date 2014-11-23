use serialize::json::{Json, ToJson};
use std::ops;
use query::term_type as ty;

// NOTE: Would a phantom type on Datum and the other structs in this module
// improve safety enough to justify the complexity and loss of expressiveness?
// I think in that case we might need an AnyDatum.
pub struct Datum(Json);

impl ToJson for Datum {
    fn to_json(&self) -> Json {
        let Datum(ref j) = *self;
        j.clone()
    }
}

pub fn expr<T: ToJson>(x: T) -> Datum {
    Datum(x.to_json())
}

// We could impl Query here but Datum queries are weird and non-standard e.g.
// for r::expr(420) the term is supposed to be just `1` on its own, not
// something like [1, [420]], so the Query trait won't work without making the
// common case annoying/wasteful (i.e. by making args() return Option<Vec>)
// In practice this probably isn't a huge deal. Who wants to use RethinkDB as
// the identity function anyway?

query! {
    Add -> Json { // NOTE: we know it's Number -> Number or String -> String, so...
        lhs: Json,
        rhs: Json
    } ty::ADD
}

impl<T: ToJson> ops::Add<T, Add> for Datum {
    fn add(&self, rhs: &T) -> Add {
        Add {
            lhs: self.to_json(),
            rhs: rhs.to_json()
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
        assert_eq!((r::expr(vec![1i32, 2, 3, 4])).to_json(), json!([1, 2, 3, 4]));
    }

    #[test]
    fn test_ops() {
        assert_eq!((r::expr(1i32) + 10i32).to_json(), json!([24, [1, 10]]));
    }
}
