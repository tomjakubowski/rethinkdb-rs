#![crate_id="restink-example#0.1.0"]
#![crate_type="bin"]

extern crate restink;

use std::io::net::ip::SocketAddr;

pub fn main() {
    use std::str;

    let addr = from_str::<SocketAddr>("127.0.0.1:28015").expect("your address is garbage");
    let mut conn = restink::connect(addr).unwrap();

    let buf = ~"[1,666,[62,[]],{\"db\":[14,[\"test\"]]}]";
    let res = conn.execute_raw(buf.into_bytes());

    match res {
        Ok(buf) => { println!("got res {}", str::from_utf8(buf).unwrap()); },
        _ => { println!("error :("); }
    }
}
