extern crate serialize;

use serialize::json;
use serialize::json::{ToJson};

enum TermType {
    TableCreate = 60,
    TableDrop = 61,
    TableList = 62
}

impl ToJson for TermType {
    fn to_json(&self) -> json::Json {
        json::Number(*self as f64)
    }
}

pub fn table_create(name: &str) -> json::Json {
    let args = json::List(~[json::String(name.to_owned())]);
    json::List(~[TableCreate.to_json(), args])
}

pub fn table_drop(name: &str) -> json::Json {
    let args = json::List(~[json::String(name.to_owned())]);
    json::List(~[TableDrop.to_json(), args])
}

pub fn table_list() -> json::Json {
    json::List(~[TableList.to_json()])
}
