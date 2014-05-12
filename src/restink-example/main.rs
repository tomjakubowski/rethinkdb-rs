#![crate_id="restink-example#0.1.0"]
#![crate_type="bin"]

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
    name: ~str
}

impl Employee {
    pub fn new(name: &str) -> Employee {
        Employee { id: None, name: name.to_owned() }
    }
}

impl ToJson for Employee {
    fn to_json(&self) -> json::Json {
        let mut e = box TreeMap::new();
        e.insert("name".to_owned(), self.name.to_json());
        json::Object(e)
    }
}

pub fn main() {
    use r = restink::query;
    use std::io::net::ip::SocketAddr;

    let addr = from_str::<SocketAddr>("127.0.0.1:28015").unwrap();
    let mut conn = restink::connect(addr).unwrap();

    let bob = Employee::new("Bob");

    println!("list tables {}", r::db("test").table_list().run(&mut conn));
    println!("create table {}", r::db("test").table_create("employees").run(&mut conn));

    println!("insert document {}",
             r::db("test").table("employees").insert(bob.to_json()).run(&mut conn));

    println!("show table {}", r::db("test").table("employees").run(&mut conn));
    println!("drop table {}", r::db("test").table_drop("employees").run(&mut conn));

    println!("list tables (should fail) {}", r::db("testing").table_list().run(&mut conn));
}
