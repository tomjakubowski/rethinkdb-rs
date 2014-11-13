pub use super::term::{mod, Func, FuncType};

use serialize::{Decodable, Decoder};
use serialize::json;
use serialize::json::ToJson;

#[deriving(Show)]
pub struct Document(pub json::Json);

pub struct Database {
    term: Func<json::Json>
}

impl Database {
    pub fn table_create(self, name: &str) -> Func<()> {
        internal::table_create(name, Some(self))
    }

    pub fn table_drop(self, name: &str) -> Func<()> {
        internal::table_drop(name, Some(self))
    }

    pub fn table_list(self) -> Func<Vec<String>> {
        internal::table_list(Some(self))
    }

    pub fn table(self, name: &str) -> Table {
        internal::table(name, Some(self))
    }
}

impl ToJson for Database {
    fn to_json(&self) -> json::Json { self.term.to_json() }
}

pub fn db(name: &str) -> Database {
    let args = vec![name.to_string().to_json()];
    Database {
        term: Func::start(term::Db, args)
    }
}

pub struct Table {
    term: Func<json::Json>
}

pub fn table(name: &str) -> Table {
    internal::table(name, None)
}

impl Table {
    pub fn get(self, key: &str) -> Func<Document> {
        self.term.chain(term::Get, vec![key.to_string().to_json()])
    }

    pub fn insert(self, document: json::Json) -> Func<Writes> {
        let args = vec![document];
        self.term.chain(term::Insert, args)
    }

    pub fn index_create(self, name: &str) -> Func<()> {
        let args = vec![name.into_string().to_json()];
        self.term.chain(term::IndexCreate, args)
    }

    pub fn index_drop(self, name: &str) -> Func<()> {
        let args = vec![name.into_string().to_json()];
        self.term.chain(term::IndexDrop, args)
    }

    pub fn index_list(self) -> Func<Vec<String>> {
        self.term.chain(term::IndexList, Vec::new())
    }
}

impl ToJson for Table {
    fn to_json(&self) -> json::Json {
        self.term.to_json()
    }
}

pub fn table_create(name: &str) -> Func<()> {
    internal::table_create(name, None)
}

pub fn table_drop(name: &str) -> Func<()> {
    internal::table_drop(name, None)
}

pub fn table_list() -> Func<Vec<String>> {
    internal::table_list(None)
}

mod internal {
    use serialize::json::{ToJson};
    use super::{Database, Func, Table};
    use term;

    pub fn table(name: &str, database: Option<Database>) -> Table {
        let func_args = vec![name.to_string().to_json()];
        let term = match database {
            Some(db) => db.term.chain(term::Table, func_args),
            None => Func::start(term::Table, func_args)
        };
        Table { term: term }
    }

    pub fn table_create(name: &str, database: Option<Database>) -> Func<()> {
        let func_args = vec![name.to_string().to_json()];
        match database {
            Some(db) => db.term.chain(term::TableCreate, func_args),
            None => Func::start(term::TableCreate, func_args)
        }
    }

    pub fn table_drop(name: &str, database: Option<Database>) -> Func<()> {
        let func_args = vec![name.to_string().to_json()];
        match database {
            Some(db) => db.term.chain(term::TableDrop, func_args),
            None => Func::start(term::TableDrop, func_args)
        }
    }

    pub fn table_list(database: Option<Database>) -> Func<Vec<String>> {
        let func_args = vec![];
        match database {
            Some(db) => db.term.chain(term::TableList, func_args),
            None => Func::start(term::TableList, func_args)
        }
    }
}

#[deriving(Show)]
pub struct Writes {
    pub deleted: u64,
    pub errors: u64,
    pub inserted: u64,
    pub replaced: u64,
    pub skipped: u64,
    pub unchanged: u64,
    pub generated_keys: Vec<String>
}

impl<D: Decoder<E>, E> Decodable<D, E> for Writes {
    fn decode(d: &mut D) -> Result<Writes, E> {
        d.read_struct("Writes", 7, |d| {
            Ok(Writes {
                deleted: try!(d.read_struct_field("deleted", 0, Decodable::decode)),
                errors: try!(d.read_struct_field("errors", 1, Decodable::decode)),
                inserted: try!(d.read_struct_field("inserted", 2, Decodable::decode)),
                replaced: try!(d.read_struct_field("replaced", 3, Decodable::decode)),
                skipped: try!(d.read_struct_field("skipped", 4, Decodable::decode)),
                unchanged: try!(d.read_struct_field("unchanged", 5, Decodable::decode)),
                generated_keys: {
                    match d.read_struct_field("generated_keys", 6, Decodable::decode) {
                        Ok(opt) => opt,
                        Err(_) => Vec::new()
                    }
                }
            })
        })
    }
}
