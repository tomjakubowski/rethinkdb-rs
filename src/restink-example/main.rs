#![crate_id = "restink-example#0.1.0"]
#![crate_type = "bin"]

extern crate collections;
extern crate restink;
extern crate serialize;

use collections::TreeMap;
use serialize::json;
use serialize::json::ToJson;

use restink::Runnable;

#[deriving(Decodable, Encodable, Show)]
struct Employee {
    id: Option<StrBuf>,
    name: StrBuf
}

impl Employee {
    pub fn new(name: &str) -> Employee {
        Employee { id: None, name: name.to_strbuf() }
    }
}

impl ToJson for Employee {
    fn to_json(&self) -> json::Json {
        let mut e = box TreeMap::new();
        match self.id {
            Some(ref id) => {
                e.insert("id".to_strbuf(), id.to_json());
            },
            _ => {
            }
        };

        e.insert("name".to_strbuf(), self.name.to_json());
        json::Object(e)
    }
}

pub fn main() {
    use r = restink::query;

    let mut conn = restink::connect("127.0.0.1", 28015).unwrap();

    let bob = Employee::new("Bob");
    println!("Bob: {}", bob.to_json());

    println!("list tables {}", r::db("test").table_list().run(&mut conn));
    println!("create table {}", r::db("test").table_create("employees").run(&mut conn));

    let writes = r::db("test").table("employees").insert(bob.to_json()).run(&mut conn);
    let writes = writes.unwrap();
    println!("insert document {}", writes);

    let key = writes.generated_keys.get(0);
    println!("get document @ {} {}", key,
             r::db("test").table("employees").get(key.as_slice()).run(&mut conn));

    println!("drop table {}", r::db("test").table_drop("employees").run(&mut conn));

    println!("list tables (should fail) {}", r::db("testing").table_list().run(&mut conn));
}
