#![crate_id="restink-example#0.1.0"]
#![crate_type="bin"]

extern crate restink;

pub fn main() {
    use r = restink::query;
    use std::io::net::ip::SocketAddr;

    let addr = from_str::<SocketAddr>("127.0.0.1:28015").expect("your address is garbage");
    let mut conn = restink::connect(addr).unwrap();

    println!("{}", conn.run(r::db("test").table_list()));
    println!("{}", conn.run(r::db("test").table_create("HELLO")));
    println!("{}", conn.run(r::db("test").table_list()));
    println!("{}", conn.run(r::db("test").table_drop("HELLO")));
    println!("{}", conn.run(r::db("test").table_list()));
    println!("{}", conn.run(r::db("testing").table_list()));
}
