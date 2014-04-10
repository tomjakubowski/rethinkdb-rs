use std::fmt;

use std::io::{BufferedStream, IoResult, IoError};
use std::io::net::tcp::{TcpStream};
use std::io::net::ip::{SocketAddr};

static magic_number: i32 = 0x5e72273a;

pub struct Connection {
    stream: BufferedStream<TcpStream>
}

impl fmt::Show for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "Connection")
    }
}

impl Connection {
    // like read_to_end, but stops when a 0 is read.
    fn read_to_null(&mut self) -> IoResult<~[u8]> {
        let mut buf = ~[];
        let mut x = try!(self.stream.read_byte());
        while x != 0 {
            buf.push(x);
            x = try!(self.stream.read_byte());
        }
        Ok(buf)
    }
    fn write_handshake(&mut self) -> IoResult<()> {
        // TODO: actually accept an optional authorization key
        let api_key_len = 0;
        try!(self.stream.write_le_i32(magic_number));
        self.stream.write_le_i32(api_key_len)
    }
}

fn other_io_error(desc: &'static str, detail: Option<~str>) -> IoError {
    use std::io;
    IoError { kind: io::OtherIoError, desc: desc, detail: detail }
}

pub fn connect(address: SocketAddr) -> IoResult<Connection> {
    use std::str;

    let stream = try!(TcpStream::connect(address));
    let mut conn = Connection { stream: BufferedStream::new(stream) };
    try!(conn.write_handshake());
    try!(conn.stream.flush());
    let response = try!(conn.read_to_null());
    match str::from_utf8(response) {
        Some("SUCCESS") => {
            println!("connected + shook hands! :)");
        },
        // FIXME: should restink have its own Result + Error types?
        Some(other) => {
            let desc = "RethinkDB Handshake Error";
            return Err(other_io_error(desc, Some(other.into_owned())));
        },
        None => {
            let desc = "RethinkDB Handshake Error";
            let detail = ~"couldn't read response as UTF-8 string";
            return Err(other_io_error(desc, Some(detail)));
        }
    };
    Ok(conn)
}
