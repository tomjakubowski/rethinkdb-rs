use from_response::FromResponse;
use net;
use serialize::json::{mod, ToJson};

use RdbResult;

pub trait Term {
    // FIXME: maybe use an associated constant for the TermType
    fn args(&self) -> Vec<json::Json>; // FIXME: Args input type? rust/#17388 is problem
    // FIXME: figure out opt_args design
    fn opt_args(&self) -> Option<json::JsonObject> {
        None
    }
}

// Unfortunately, due to coherence rules we cannot do:
// impl<T: Term> ToJson for T {
//     fn to_json(&self) -> json::Json {
//         ...
//     }
// }
/// A term which can be executed in RethinkDB as a query.
/// The input type to this trait will become an output type once a Rust bug is fixed.
pub trait Query: ToJson + Term {
    /// A type which can be decoded from the successful response of executing this query.
    type R: FromResponse;

    fn run(self, conn: &mut net::Connection) -> RdbResult< <Self as Query>::R> {
        net::run(conn, self.to_json()).and_then(FromResponse::from_response)
    }
}

// FIXME: need to be able to define a term with multiple variants, for example:
// enum TableCreate {
//     TableCreateNormal { db: Db, name: String },
//     TableCreateDefaultDb { name: String}
// }
macro_rules! term {
    ($name:ident -> $resp:ty {
        $($field:ident: $ty:ty),*
    } $term_ty:expr) => {
        pub struct $name {
            $($field: $ty),*
        }

        impl Term for $name {
            fn args(&self) -> Vec<json::Json> {
                vec![$(self.$field.to_json()),*]
            }
        }

        impl ToJson for $name {
            fn to_json(&self) -> ::serialize::json::Json {
                match self.opt_args() {
                    Some(opt_args) => {
                        ($term_ty.to_json(), self.args(), opt_args).to_json()
                    }
                    None => {
                        ($term_ty.to_json(), self.args()).to_json()
                    }
                }
            }
        }

        impl Query for $name {
            type R = $resp;
        }
    }
}

term! {
    Db -> () {
        name: String
    } ty::DB
}

impl Db {
    pub fn table_create(self, name: &str) -> TableCreate {
        TableCreate { name: name.into_string(), db: self }
    }

    pub fn table_drop(self, name: &str) -> TableDrop {
        TableDrop { name: name.into_string(), db: self }
    }

    pub fn table_list(self) -> TableList {
        TableList { db: self }
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
    TableCreate -> () {
        db: Db,
        name: String
    } ty::TABLE_CREATE
}

term! {
    TableDrop -> () {
        db: Db,
        name: String
    } ty::TABLE_DROP
}

term! {
    TableList -> Vec<String> {
        db: Db
    } ty::TABLE_LIST
}

term! {
    // FIXME: should return an iterator over the documents of the table
    Table -> () {
        db: Db,
        name: String
    } ty::TABLE
}

mod ty {
    pub type TermType = i64;

    pub const DB: TermType = 14;
    pub const TABLE: TermType = 15;
    // pub const GET: TermType = 16;
    // pub const INSERT: TermType = 56;
    pub const DB_CREATE: TermType = 57;
    pub const DB_DROP: TermType = 58;
    pub const TABLE_CREATE: TermType = 60;
    pub const TABLE_DROP: TermType = 61;
    pub const TABLE_LIST: TermType = 62;
    // pub const INDEX_CREATE: TermType = 75;
    // pub const INDEX_DROP: TermType = 76;
    // pub const INDEX_LIST: TermType = 77;
}

#[cfg(test)]
mod test {
    #[phase(plugin)] extern crate json_macros;

    use query2 as r;
    use serialize::json::ToJson;

    #[test]
    fn test_db() {
        assert_eq!(r::db("test").to_json(), json!([14, ["test"]]));
        assert_eq!(r::db_create("foo").to_json(), json!([57, ["foo"]]));
        assert_eq!(r::db_drop("foo").to_json(), json!([58, ["foo"]]));
        // assert_eq!(r::db("foo").table("bar").to_json(), json!([15, [[14, ["foo"]], "bar"]]));
        assert_eq!(r::db("foo").table_create("bar").to_json(), json!([60, [[14, ["foo"]], "bar"]]));
        // assert_eq!(r::db("foo").table_drop("bar").to_json(), json!([61, [[14, ["foo"]], "bar"]]));
        assert_eq!(r::db("foo").table_list().to_json(), json!([62, [[14, ["foo"]]]]));
    }

    #[test]
    fn test_table() {
        // assert_eq!(r::table("foo").to_json(), json!([15, ["foo"]]));
        // assert_eq!(r::table("test").get("deadbeef").to_json(), json!([16, [[15, ["test"]], "deadbeef"]]));
        // assert_eq!(r::table("test").insert(json!({ "foo": "bar" })).to_json(), json!([56, [[15, ["test"]], {"foo": "bar"}]]));
        // assert_eq!(r::table("test").index_create("bar").to_json(), json!([75, [[15, ["test"]], "bar"]]));
        // assert_eq!(r::table("test").index_drop("bar").to_json(), json!([76, [[15, ["test"]], "bar"]]));
        // assert_eq!(r::table("test").index_list().to_json(), json!([77, [[15, ["test"]]]]));
    }
}

