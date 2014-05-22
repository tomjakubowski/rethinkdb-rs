#![feature(default_type_params)]

extern crate collections;
extern crate serialize;

pub use self::term::{Func, FuncType};

use serialize::{Decodable, Decoder};
use serialize::json;
use serialize::json::ToJson;

pub type Datum = json::Json;

mod term {
    use super::Datum;

    use serialize::json;
    use serialize::json::{Json,ToJson};

    pub struct Func<Out=Json> {
        func_type: FuncType,
        prev: Option<Json>,
        args: Vec<Datum>,
        opt_args: Option<json::Object>
    }

    impl<Out> Func {
        pub fn chain(self, func_type: FuncType, args: Vec<Datum>) -> Func<Out> {
            Func {
                func_type: func_type,
                prev: Some(self.to_json()),
                args: args,
                opt_args: None
            }
        }
    }

    impl Func {
        pub fn start(func_type: FuncType, args: Vec<Datum>) -> Func {
            Func {
                func_type: func_type,
                prev: None,
                args: args,
                opt_args: None
            }
        }
    }

    impl<T> ToJson for Func<T> {
        fn to_json(&self) -> json::Json {
            use collections::TreeMap;

            let Func { func_type, ref prev, ref args, ref opt_args } = *self;
            let mut term_args = match *prev {
                Some(ref f) => { vec![f.to_json()] },
                None => { vec![] }
            };
            term_args.push_all(args.as_slice());

            let term_opt_args = match *opt_args {
                None => json::Object(box TreeMap::new()),
                Some(ref ob) => json::Object(box ob.clone())
            };

            json::List(vec![func_type.to_json(), term_args.to_json(), term_opt_args.to_json()])
        }
    }

    pub enum FuncType {
        Db = 14,
        Table = 15,
        Get = 16,
        Insert = 56,
        TableCreate = 60,
        TableDrop = 61,
        TableList = 62
    }

    impl ToJson for FuncType {
        fn to_json(&self) -> json::Json {
            json::Number(*self as f64)
        }
    }

}


pub struct Database {
    term: Func<json::Json>
}

impl Database {
    pub fn table_create(self, name: &str) -> Func {
        internal::table_create(name, Some(self))
    }

    pub fn table_drop(self, name: &str) -> Func {
        internal::table_drop(name, Some(self))
    }

    pub fn table_list(self) -> Func {
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
    let args = vec![name.to_strbuf().to_json()];
    Database {
        term: Func::start(term::Db, args)
    }
}

pub struct Table {
    term: Func
}

pub fn table(name: &str) -> Table {
    internal::table(name, None)
}

impl Table {
    pub fn get(self, key: &str) -> Func {
        self.term.chain(term::Get, vec![key.to_strbuf().to_json()])
    }

    pub fn insert(self, document: json::Json) -> Func<Writes> {
        let args = vec![document];
        self.term.chain(term::Insert, args)
    }
}

impl ToJson for Table {
    fn to_json(&self) -> json::Json {
        self.term.to_json()
    }
}

pub fn table_create(name: &str) -> Func {
    internal::table_create(name, None)
}

pub fn table_drop(name: &str) -> Func {
    internal::table_drop(name, None)
}

pub fn table_list() -> Func {
    internal::table_list(None)
}

mod internal {
    use serialize::json::{ToJson};
    use super::{Database, Func, Table, term};

    pub fn table(name: &str, database: Option<Database>) -> Table {
        let func_args = vec![name.to_strbuf().to_json()];
        let term = match database {
            Some(db) => db.term.chain(term::Table, func_args),
            None => Func::start(term::Table, func_args)
        };
        Table { term: term }
    }

    pub fn table_create(name: &str, database: Option<Database>) -> Func {
        let func_args = vec![name.to_strbuf().to_json()];
        match database {
            Some(db) => db.term.chain(term::TableCreate, func_args),
            None => Func::start(term::TableCreate, func_args)
        }
    }

    pub fn table_drop(name: &str, database: Option<Database>) -> Func {
        let func_args = vec![name.to_strbuf().to_json()];
        match database {
            Some(db) => db.term.chain(term::TableDrop, func_args),
            None => Func::start(term::TableDrop, func_args)
        }
    }

    pub fn table_list(database: Option<Database>) -> Func {
        let func_args = vec![];
        match database {
            Some(db) => db.term.chain(term::TableList, func_args),
            None => Func::start(term::TableList, func_args)
        }
    }
}

#[deriving(Show)]
pub struct Writes {
    pub deleted: uint,
    pub errors: uint,
    pub inserted: uint,
    pub replaced: uint,
    pub skipped: uint,
    pub unchanged: uint,
    pub generated_keys: Vec<StrBuf>
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
                generated_keys: match d.read_struct_field("generated_keys",
                                                          6u, |d| Decodable::decode(d)) {
                    Ok(opt) => opt,
                    Err(_) => Vec::new()
                }
            })
        })
    }
}
