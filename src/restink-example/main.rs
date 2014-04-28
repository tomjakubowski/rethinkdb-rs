#![crate_id="restink-example#0.1.0"]
#![crate_type="bin"]

extern crate restink;

pub fn main() {
    use r = restink::query;
    use std::io::net::ip::SocketAddr;

    let addr = from_str::<SocketAddr>("127.0.0.1:28015").expect("your address is garbage");
    let mut conn = restink::connect(addr).unwrap();

    conn.run(r::table_list());
    conn.run(r::table_create("HELLO"));
    conn.run(r::table_list());
    conn.run(r::table_drop("HELLO"));
    conn.run(r::table_list());
}
