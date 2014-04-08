use std::fmt;

use std::io::IoResult;
use std::io::net::tcp::{TcpStream};
use std::io::net::ip::{SocketAddr};

pub struct Connection {
    stream: TcpStream
}

impl fmt::Show for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "Connection")
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("conn going out of scope!");
    }
}

pub fn connect(address: SocketAddr) -> IoResult<Connection> {
    TcpStream::connect(address).and_then(|mut sock| {
        sock.write_str("HELLO WORLD\n").and(Ok(Connection { stream: sock }))
    })
}
