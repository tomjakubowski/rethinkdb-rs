extern crate serialize;

use serialize::json;
use serialize::json::{ToJson};

mod term {
    use serialize::json;
    use serialize::json::{ToJson};

    pub enum Type {
        Db = 14,
        TableCreate = 60,
        TableDrop = 61,
        TableList = 62
    }

    impl ToJson for Type {
        fn to_json(&self) -> json::Json {
            json::Number(*self as f64)
        }
    }
}

pub struct Database {
    term: json::Json
}

impl Database {
    pub fn table_create(self, name: &str) -> json::Json {
        internal::table_create(name, Some(self))
    }

    pub fn table_drop(self, name: &str) -> json::Json {
        internal::table_drop(name, Some(self))
    }

    pub fn table_list(self) -> json::Json {
        internal::table_list(Some(self))
    }
}

pub fn db(name: &str) -> Database {
    let args = json::List(~[name.to_owned().to_json()]);
    Database { term: json::List(~[term::Db.to_json(), args]) }
}

mod internal {
    use j = serialize::json;
    use serialize::json::{ToJson};
    use super::{Database, term};

    pub fn table_create(name: &str, db: Option<Database>) -> j::Json {
        // vvv this is screaming for a macro
        // build_args!(name, db, ...) =>
        let args = match db {
            Some(database) => {
                j::List(~[database.term, name.to_owned().to_json()])
            },
            None => {
                j::List(~[name.to_owned().to_json()])
            }
        };
        j::List(~[term::TableCreate.to_json(), args])
    }

    pub fn table_drop(name: &str, db: Option<Database>) -> j::Json {
        let args = match db {
            Some(database) => {
                j::List(~[database.term, name.to_owned().to_json()])
            },
            None => {
                j::List(~[name.to_owned().to_json()])
            }
        };
        j::List(~[term::TableDrop.to_json(), args])
    }

    pub fn table_list(db: Option<Database>) -> j::Json {
        let args = match db {
            Some(database) => {
                j::List(~[database.term])
            },
            None => {
                j::List(~[])
            }
        };
        j::List(~[term::TableList.to_json(), args])
    }
}

pub fn table_create(name: &str) -> json::Json {
    internal::table_create(name, None)
}

pub fn table_drop(name: &str) -> json::Json {
    internal::table_drop(name, None)
}

pub fn table_list() -> json::Json {
    internal::table_list(None)
}
