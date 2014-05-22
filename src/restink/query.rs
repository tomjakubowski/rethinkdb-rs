extern crate collections;
extern crate serialize;

pub use self::term::{Func, FuncType};

use serialize::json;
use serialize::json::ToJson;

pub type Datum = json::Json;

mod term {
    use super::Datum;

    use serialize::json;
    use serialize::json::{ToJson};

    pub struct Func {
        func_type: FuncType,
        prev: Option<Box<Func>>,
        args: Vec<Datum>,
        opt_args: Option<json::Object>
    }

    impl Func {
        pub fn chain(self, func_type: FuncType, args: Vec<Datum>) -> Func {
            Func {
                func_type: func_type,
                prev: Some(box self),
                args: args,
                opt_args: None
            }
        }

        pub fn start(func_type: FuncType, args: Vec<Datum>) -> Func {
            Func {
                func_type: func_type,
                prev: None,
                args: args,
                opt_args: None
            }
        }
    }

    impl ToJson for Func {
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
    term: Func
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
    pub fn insert(self, document: json::Json) -> Func {
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
