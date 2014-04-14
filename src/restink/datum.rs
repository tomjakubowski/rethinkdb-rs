extern crate collections;
use collections::HashMap;

pub type AssocPair<T> = HashMap<~str, T>;

// todo: phantom types?
#[deriving(Show, Eq, Clone)]
pub enum Datum {
    RNull,
    RBool(bool),
    RNum(f64),
    RStr(~str),
    RArray(Vec<~Datum>),         // should this be Vec<~Datum> or ~Vec<Datum> ?
    RObject(AssocPair<~Datum>)   // this too
}

pub trait ToDatum {
    fn to_datum(self) -> Datum;
}

impl<T: ToDatum> ToDatum for Option<T> {
    fn to_datum(self) -> Datum {
        match self {
            Some(x) => x.to_datum(),
            None => RNull
        }
    }
}

impl ToDatum for bool {
    fn to_datum(self) -> Datum { RBool(self) }
}

impl ToDatum for f64 {
    fn to_datum(self) -> Datum { RNum(self) }
}

impl<'a> ToDatum for &'a str {
    fn to_datum(self) -> Datum { RStr(self.clone().to_owned()) }
}

// to_datum() for ~str, Vec<Datum>, and AssocPair<Datum> all move.
impl ToDatum for ~str {
    fn to_datum(self) -> Datum { RStr(self) }
}

impl<T: ToDatum> ToDatum for Vec<T> {
    fn to_datum(self) -> Datum {
        let mut mapped = self.move_iter().map(|x| ~x.to_datum());
        RArray(mapped.collect())
    }
}

impl<T: ToDatum> ToDatum for HashMap<~str, T> {
    fn to_datum(self) -> Datum {
        let mut mapped = self.move_iter().map(|(k, v)| (k, ~v.to_datum()));
        RObject(mapped.collect())
    }
}

#[cfg(test)]
mod test {
    use super::{ToDatum, RNull, RBool, RNum, RStr, RArray, RObject};

    #[test]
    fn test_bool_to_datum() {
        assert!(true.to_datum() == RBool(true));
    }

    #[test]
    fn test_num_to_datum() {
        assert!((1.0).to_datum() == RNum(1.0));
    }

    #[test]
    fn test_option_to_datum() {
        assert!(Some(true).to_datum() == RBool(true));
        assert!(None::<bool>.to_datum() == RNull);
    }

    #[test]
    fn test_owned_str_to_datum() {
        assert!((~"foo").to_datum() == RStr(~"foo"));
    }

    #[test]
    fn test_static_str_to_datum() {
        assert!("foo".to_datum() == RStr(~"foo"));
    }

    #[test]
    fn test_homogenous_vec_to_datum() {
        let x = vec!("foo", "bar", "baz");
        assert!(x.to_datum() == RArray(vec!(~RStr(~"foo"), ~RStr(~"bar"), ~RStr(~"baz"))));
    }

    #[test]
    fn test_homogenous_assocpair_to_datum() {
        use super::AssocPair;
        let x: AssocPair<bool> = vec!((~"foo", true), (~"bar", false)).move_iter().collect();
        x.to_datum();
    }
}
