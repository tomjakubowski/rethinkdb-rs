#![crate_id="restink-example#0.1.0"]
#![crate_type="bin"]

extern crate collections;
extern crate restink;
extern crate serialize;

use collections::treemap::{TreeMap};

pub fn main() {
    use j = serialize::json;

    use std::io::net::ip::SocketAddr;
    use std::str;

    let addr = from_str::<SocketAddr>("127.0.0.1:28015").expect("your address is garbage");
    let mut conn = restink::connect(addr).unwrap();

    let mut optargs = ~TreeMap::new();
    optargs.insert(~"db", j::List(~[j::Number(14.),
                                    j::List(~[j::String(~"test")])]));
    let optargs = j::Object(optargs);

    let args = j::List(~[j::Number(62.), j::List(~[])]);
    let query = ~j::List(~[j::Number(1.), args, optargs]);

    // let buf = ~"[1,[62,[]],{\"db\":[14,[\"test\"]]}]";
    let res = conn.execute_json(query);

    match res {
        Ok(buf) => {
            println!("got res {}", str::from_utf8(buf.as_slice()).unwrap());
        },
        _ => { println!("error :("); }
    }
}
