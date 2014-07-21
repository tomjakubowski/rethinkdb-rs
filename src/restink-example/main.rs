#![crate_name = "restink-example"]
#![crate_type = "bin"]

extern crate collections;
extern crate restink;
extern crate serialize;

use collections::TreeMap;
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
    use r = restink::query;

    let mut conn = restink::connect("127.0.0.1", 28015).unwrap();

    let bob = Employee::new("Bob");
    println!("Bob: {}", bob.to_json());

    println!("create table {}", r::db("test").table_create("employees").run(&mut conn));

    println!("create index {}",
             r::db("test").table("employees").index_create("name").run(&mut conn));

    let writes = r::db("test").table("employees").insert(bob.to_json()).run(&mut conn);
    let writes = writes.unwrap();
    println!("insert document {}", writes);

    let key: &str = writes.generated_keys[0].as_slice();
    println!("get document @ {} {}", key,
             r::db("test").table("employees").get(key).run(&mut conn));

    println!("list indexes {}",
             r::db("test").table("employees").index_list().run(&mut conn));

    println!("drop index {}",
             r::db("test").table("employees").index_drop("name").run(&mut conn));

    println!("list indexes {}",
             r::db("test").table("employees").index_list().run(&mut conn));

}
