extern crate serialize;

pub use self::term::Func;

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
        pub fn new(prev: Func, func_type: FuncType, args: Vec<Datum>) -> Func {
            Func {
                func_type: func_type,
                prev: Some(box prev),
                args: args,
                opt_args: None
            }
        }

        pub fn new_chain(func_type: FuncType, args: Vec<Datum>) -> Func {
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
            let Func { func_type, ref prev, ref args, ref opt_args } = *self;
            let mut term_args = match *prev {
                Some(ref f) => { vec![f.to_json()] },
                None => { vec![] }
            };
            term_args.push_all(args.as_slice());
            json::List(vec![func_type.to_json(), term_args.to_json(), opt_args.to_json()])
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
    pub fn table_create(self, name: &str) -> Datum {
        internal::table_create(name, Some(self))
    }

    pub fn table_drop(self, name: &str) -> Datum {
        internal::table_drop(name, Some(self))
    }

    pub fn table_list(self) -> Datum {
        internal::table_list(Some(self))
    }

    pub fn table(self, name: &str) -> Table {
        internal::table(name, Some(self))
    }
}

pub fn db(name: &str) -> Database {
    let args = json::List(vec![name.to_owned().to_json()]);
    Database { term: json::List(vec![term::Db.to_json(), args]) }
}

pub struct Table {
    // FIXME term as public field is a temporary workaround
    pub term: Datum
}

pub fn table(name: &str) -> Table {
    internal::table(name, None)
}

impl Table {
    pub fn insert(self, doc: Datum) -> Datum {
        let args = json::List(vec![self.term, doc]);
        json::List(vec![term::Insert.to_json(), args])
    }
}

impl ToJson for Table {
    fn to_json(&self) -> json::Json {
        self.term.clone()
    }
}

mod internal {
    use j = serialize::json;
    use serialize::json::{ToJson};
    use super::{Database, Datum, Table, term};

    pub fn table(name: &str, db: Option<Database>) -> Table {
        let args = match db {
            Some(database) => j::List(vec![database.term, name.to_owned().to_json()]),
            None => j::List(vec![name.to_owned().to_json()])
        };
        Table {
            term: j::List(vec![term::Table.to_json(), args])
        }
    }

    pub fn table_create(name: &str, db: Option<Database>) -> Datum {
        // vvv this is screaming for a macro
        // build_args!(name, db, ...) =>
        let args = match db {
            Some(database) => {
                j::List(vec![database.term, name.to_owned().to_json()])
            },
            None => {
                j::List(vec![name.to_owned().to_json()])
            }
        };
        j::List(vec![term::TableCreate.to_json(), args])
    }

    pub fn table_drop(name: &str, db: Option<Database>) -> Datum {
        let args = match db {
            Some(database) => {
                j::List(vec![database.term, name.to_owned().to_json()])
            },
            None => {
                j::List(vec![name.to_owned().to_json()])
            }
        };
        j::List(vec![term::TableDrop.to_json(), args])
    }

    pub fn table_list(db: Option<Database>) -> Datum {
        let args = match db {
            Some(database) => {
                j::List(vec![database.term])
            },
            None => {
                j::List(vec![])
            }
        };
        j::List(vec![term::TableList.to_json(), args])
    }
}

pub fn table_create(name: &str) -> Datum {
    internal::table_create(name, None)
}

pub fn table_drop(name: &str) -> Datum {
    internal::table_drop(name, None)
}

pub fn table_list() -> Datum {
    internal::table_list(None)
}
