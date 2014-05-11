#![crate_id="restink-example#0.1.0"]
#![crate_type="bin"]

extern crate collections;
extern crate restink;
extern crate serialize;

use collections::TreeMap;
use serialize::json;
use serialize::json::ToJson;

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

    let addr = from_str::<SocketAddr>("127.0.0.1:28015").expect("your address is garbage");
    let mut conn = restink::connect(addr).unwrap();

    let bob = Employee::new("Bob");

    println!("{}", conn.run(r::db("test").table_list()));
    println!("{}", conn.run(r::db("test").table_create("employees")));

    println!("{}", conn.run(r::db("test").table("employees").insert(bob.to_json())));
    println!("{}", conn.run(r::db("test").table("employees").term));
    println!("{}", conn.run(r::db("test").table_drop("employees")));
    println!("{}", conn.run(r::db("testing").table_list()));
}
