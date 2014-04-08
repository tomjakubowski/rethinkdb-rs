#![crate_id="restink-example#0.1.0"]
#![crate_type="bin"]

extern crate restink;

use std::io::net::ip::SocketAddr;

pub fn main() {
    let addr = from_str::<SocketAddr>("127.0.0.1:28015").expect("your address is garbage");
    let conn = restink::connect(addr);

    println!("connection {}", conn);
}
