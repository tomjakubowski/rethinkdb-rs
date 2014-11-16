use from_response::FromResponse;
use net;
use serialize::{Decodable, Decoder};
use serialize::json::{mod, ToJson};

use RdbResult;

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
    ($name:ident -> $resp:ty {
        $($field:ident: $ty:ty),*
    } $term_ty:expr) => {
        #[deriving(Show)]
        pub struct $name {
            $($field: $ty),*
        }

        impl ::query::Term for $name {
            fn args(&self) -> Vec<::serialize::json::Json> {
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
                match *self {
                    $($variant $({ $(ref $field),+ })* => {
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

term! {
    Db -> () {
        name: String
    } ty::DB
}

impl Db {
    pub fn table(self, name: &str) -> Table {
        Table2 { db: self, name: name.into_string() }
    }

    pub fn table_create(self, name: &str) -> TableCreate {
        TableCreate2 { name: name.into_string(), db: self }
    }

    pub fn table_drop(self, name: &str) -> TableDrop {
        TableDrop2 { name: name.into_string(), db: self }
    }

    pub fn table_list(self) -> TableList {
        TableList1 { db: self }
    }
}

pub fn db(name: &str) -> Db {
    Db { name: name.into_string() }
}

term! {
    DbCreate -> () {
        name: String
    } ty::DB_CREATE
}

pub fn db_create(name: &str) -> DbCreate {
    DbCreate { name: name.into_string() }
}

term! {
    DbDrop -> () {
        name: String
    } ty::DB_DROP
}

pub fn db_drop(name: &str) -> DbDrop {
    DbDrop { name: name.into_string() }
}

term! {
    enum TableCreate -> () {
        TableCreate1 { name: String },
        TableCreate2 { db: Db, name: String }
    } ty::TABLE_CREATE
}

pub fn table_create(name: &str) -> TableCreate {
    TableCreate1 { name: name.into_string() }
}

term! {
    enum TableDrop -> () {
        TableDrop1 { name: String},
        TableDrop2 { db: Db, name: String }
    } ty::TABLE_DROP
}

pub fn table_drop(name: &str) -> TableDrop {
    TableDrop1 { name: name.into_string() }
}

term! {
    enum TableList -> Vec<String> {
        TableList1 { db: Db },
        TableList0
    } ty::TABLE_LIST
}

pub fn table_list() -> TableList {
    TableList0
}

term! {
    // FIXME: should return an iterator over the documents of the table
    enum Table -> () {
        Table1 { name: String },
        Table2 { db: Db, name: String }
    } ty::TABLE
}

pub fn table(name: &str) -> Table {
    Table1 { name: name.into_string() }
}

impl Table {
    pub fn get(self, key: &str) -> Get {
        Get { table: self, key: key.into_string() }
    }

    pub fn insert(self, document: json::Json) -> Insert {
        Insert { table: self, document: document }
    }

    pub fn index_create(self, name: &str) -> IndexCreate {
        IndexCreate { table: self, name: name.into_string() }
    }

    pub fn index_drop(self, name: &str) -> IndexDrop {
        IndexDrop { table: self, name: name.into_string() }
    }

    pub fn index_list(self) -> IndexList {
        IndexList { table: self }
    }
}

term! {
    Get -> json::Json {
        table: Table,
        key: String
    } ty::GET
}

term! {
    Insert -> Writes {
        table: Table,
        document: json::Json
    } ty::INSERT
}

term! {
    IndexCreate -> () {
        table: Table,
        name: String
    } ty::INDEX_CREATE
}

term! {
    IndexDrop -> () {
        table: Table,
        name: String
    } ty::INDEX_DROP
}

term! {
    IndexList -> Vec<String> {
        table: Table
    } ty::INDEX_LIST
}

// FIXME: this perhaps belongs somewhere else
#[deriving(Show)]
pub struct Writes {
    pub deleted: uint,
    pub errors: uint,
    pub inserted: uint,
    pub replaced: uint,
    pub skipped: uint,
    pub unchanged: uint,
    pub generated_keys: Vec<String>
}

impl<D: Decoder<E>, E> Decodable<D, E> for Writes {
    fn decode(d: &mut D) -> Result<Writes, E> {
        d.read_struct("Writes", 7u, |d| {
            Ok(Writes {
                deleted: try!(d.read_struct_field("deleted", 0u, |d| Decodable::decode(d))),
                errors: try!(d.read_struct_field("errors", 1u, |d| Decodable::decode(d))),
                inserted: try!(d.read_struct_field("inserted", 2u, |d| Decodable::decode(d))),
                replaced: try!(d.read_struct_field("replaced", 3u, |d| Decodable::decode(d))),
                skipped: try!(d.read_struct_field("skipped", 4u, |d| Decodable::decode(d))),
                unchanged: try!(d.read_struct_field("unchanged", 5u, |d| Decodable::decode(d))),
                generated_keys: {
                    match d.read_struct_field("generated_keys", 6u, |d| Decodable::decode(d)) {
                        Ok(opt) => opt,
                        Err(_) => Vec::new()
                    }
                }
            })
        })
    }
}

mod ty {
    pub type TermType = i64;

    pub const DB: TermType = 14;
    pub const TABLE: TermType = 15;
    pub const GET: TermType = 16;
    pub const INSERT: TermType = 56;
    pub const DB_CREATE: TermType = 57;
    pub const DB_DROP: TermType = 58;
    pub const TABLE_CREATE: TermType = 60;
    pub const TABLE_DROP: TermType = 61;
    pub const TABLE_LIST: TermType = 62;
    pub const INDEX_CREATE: TermType = 75;
    pub const INDEX_DROP: TermType = 76;
    pub const INDEX_LIST: TermType = 77;
}

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
    }
}

