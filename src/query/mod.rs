use serialize::json::{mod, ToJson};

use from_response::FromResponse;
use net;
use RdbResult;

pub use self::db::{Db, db, db_create, db_drop, db_list};
pub use self::table::{Get, Table, table, table_create, table_drop, table_list};

pub trait Term {
    // FIXME: eventually use an associated constant for the TermType
    fn args(&self) -> Vec<json::Json>; // FIXME: Args input type? rust/#17388 is problem
    // FIXME: figure out opt_args design
    fn opt_args(&self) -> Option<json::JsonObject> {
        None
    }
}

/// A term which can be executed in RethinkDB as a query.
/// The input type to this trait will become an output type once a Rust bug is fixed.
pub trait Query<R: FromResponse>: ToJson + Term {
    /* FIXME: restore this when associated types are less broken rust/#18048
    /// A type which can be decoded from the successful response of executing this query.
    type R: FromResponse;
    */

    fn run(self, conn: &mut net::Connection) -> RdbResult<R> {
        net::run(conn, self.to_json()).and_then(FromResponse::from_response)
    }
}

// Unfortunately, due to coherence rules we cannot do a single;
// impl<T: Term> ToJson for T {
//     fn to_json(&self) -> json::Json {
//         ...
//     }
// }
macro_rules! to_json_impl {
    ($name:ident $term_ty:expr) => {
        impl ::serialize::json::ToJson for $name {
            fn to_json(&self) -> ::serialize::json::Json {
                use query::Term;
                match self.opt_args() {
                    Some(opt_args) => {
                        ($term_ty, self.args(), opt_args).to_json()
                    }
                    None => {
                        ($term_ty, self.args()).to_json()
                    }
                }
            }
        }
    }
}

macro_rules! term {
    // FIXME: a Term need not be a Query (this could be signaled by leaving off
    // the resp)
    ($name:ident -> $resp:ty ; $term_ty:expr) => {
        #[deriving(Show)]
        pub struct $name;

        impl ::query::Term for $name {
            fn args(&self) -> Vec<::serialize::json::Json> {
                vec![]
            }
        }

        to_json_impl! { $name $term_ty }

        impl ::query::Query<$resp> for $name {
            // type R = $resp;
        }
    };
    ($name:ident -> $resp:ty {
        $($field:ident: $ty:ty),*
    } $term_ty:expr) => {
        #[deriving(Show)]
        pub struct $name {
            $($field: $ty),*
        }

        impl ::query::Term for $name {
            fn args(&self) -> Vec<::serialize::json::Json> {
                use serialize::json::ToJson;
                vec![$(self.$field.to_json()),*]
            }
        }

        to_json_impl! { $name $term_ty }

        impl ::query::Query<$resp> for $name {
            // type R = $resp;
        }
    };
    // The extra $()* around the { $($field $ty) } is a hack to make that optional :(
    // How about something like $(...)? ?
    (enum $name:ident -> $resp:ty {
        $( $variant:ident $({
            $( $field:ident : $ty:ty ),+
        })*),*
    } $term_ty:expr) => {
        #[deriving(Show)]
        pub enum $name {
            $( $variant $({
                $( $field: $ty ),+
            })* ),*
        }

        impl ::query::Term for $name {
            fn args(&self) -> Vec<::serialize::json::Json> {
                use serialize::json::ToJson;
                match *self {
                    $($name::$variant $({ $(ref $field),+ })* => {
                        vec![$($($field.to_json()),+)*]
                    }),*
                }
            }
        }

        to_json_impl! { $name $term_ty }

        impl ::query::Query<$resp> for $name {
            // type R = $resp;
        }
    }
}

// FIXME: this perhaps belongs somewhere else
// FIXME: having Option<> on everything is really annoying. Can we make RDB
// always return all fields (at least the ones which are counts)?
#[deriving(Decodable, Show)]
pub struct Writes {
    pub deleted: Option<u64>,
    pub errors: Option<u64>,
    pub inserted: Option<u64>,
    pub replaced: Option<u64>,
    pub skipped: Option<u64>,
    pub unchanged: Option<u64>,
    pub generated_keys: Option<Vec<String>>,
    pub first_error: Option<String>
}

mod db;
mod table;
mod term_type;

#[cfg(test)]
mod test {
    #[phase(plugin)] extern crate json_macros;

    use query as r;
    use serialize::json::ToJson;

    #[test]
    fn test_db() {
        assert_eq!(r::db("test").to_json(), json!([14, ["test"]]));
        assert_eq!(r::db_create("foo").to_json(), json!([57, ["foo"]]));
        assert_eq!(r::db_drop("foo").to_json(), json!([58, ["foo"]]));
        assert_eq!(r::db("foo").table("bar").to_json(), json!([15, [[14, ["foo"]], "bar"]]));
        assert_eq!(r::db("foo").table_create("bar").to_json(), json!([60, [[14, ["foo"]], "bar"]]));
        assert_eq!(r::db("foo").table_drop("bar").to_json(), json!([61, [[14, ["foo"]], "bar"]]));
        assert_eq!(r::db("foo").table_list().to_json(), json!([62, [[14, ["foo"]]]]));
    }

    #[test]
    fn test_table() {
        assert_eq!(r::table("foo").to_json(), json!([15, ["foo"]]));
        assert_eq!(r::table("test").get("deadbeef").to_json(), json!([16, [[15, ["test"]], "deadbeef"]]));
        assert_eq!(r::table("test").insert(json!({ "foo": "bar" })).to_json(), json!([56, [[15, ["test"]], {"foo": "bar"}]]));
        assert_eq!(r::table("test").index_create("bar").to_json(), json!([75, [[15, ["test"]], "bar"]]));
        assert_eq!(r::table("test").index_drop("bar").to_json(), json!([76, [[15, ["test"]], "bar"]]));
        assert_eq!(r::table("test").index_list().to_json(), json!([77, [[15, ["test"]]]]));
        assert_eq!(r::table("foo").get("bar").delete().to_json(), json!([54, [[16, [[15, ["foo"]], "bar"]]]]));
    }
}

