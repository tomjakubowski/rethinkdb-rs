#![crate_name = "restink-employees"]
#![crate_type = "bin"]

extern crate restink;
extern crate serialize;

use std::collections::TreeMap;
use serialize::json;
use serialize::json::ToJson;

#[deriving(Decodable, Encodable, Show)]
struct Employee {
    id: Option<String>,
    name: String
}

impl Employee {
    pub fn new(name: &str) -> Employee {
        Employee { id: None, name: name.to_string() }
    }
}

impl ToJson for Employee {
    fn to_json(&self) -> json::Json {
        let mut e = TreeMap::new();
        match self.id {
            Some(ref id) => {
                e.insert("id".to_string(), id.to_json());
            },
            _ => {
            }
        };

        e.insert("name".to_string(), self.name.to_json());
        json::Object(e)
    }
}

pub fn main() {
    use restink::query as r;

    let mut conn = restink::connect("127.0.0.1", 28015).unwrap();
    // Example usage... we can't actually create DBs yet, so in the
    // end stick with "test" which should already exist.
    conn.use_db("phillips_broadcasting");
    conn.use_db("test");

    let jay = Employee::new("Jay Sherman");
    println!("Jay: {}", jay.to_json());
    let duke = Employee::new("Duke Phillips");
    println!("Duke: {}", duke.to_json());

    println!("create table {}", r::table_create("employees").run(&mut conn));
    println!("create index {}", r::table("employees").index_create("name").run(&mut conn));

    let writes = r::table("employees").insert(jay.to_json()).run(&mut conn);
    let writes = writes.unwrap();
    println!("insert document {}", writes);

    let writes = r::table("employees").insert(duke.to_json()).run(&mut conn);
    let writes = writes.unwrap();
    println!("insert document {}", writes);

    let key: &str = writes.generated_keys[0].as_slice();
    println!("get document @ {} {}", key, r::table("employees").get(key).run(&mut conn));
    println!("list indexes {}", r::table("employees").index_list().run(&mut conn));
    println!("drop index {}", r::table("employees").index_drop("name").run(&mut conn));
    println!("list indexes {}", r::table("employees").index_list().run(&mut conn));
    println!("table drop {}", r::table_drop("employees").run(&mut conn));
}