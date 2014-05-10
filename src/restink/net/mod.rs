#![feature(struct_variant)]

extern crate collections;
extern crate serialize;

use collections::TreeMap;

use serialize::json;
use serialize::json::{Json};

use std::fmt;
use std::io::{BufferedStream, IoResult};
use std::io::net::tcp::{TcpStream};
use std::io::net::ip::{SocketAddr};

pub use Error = self::response::Error;
pub use RdbResult = self::response::RdbResult;
pub use Response = self::response::Response;
pub use ResponseKind = self::response::ResponseKind;

mod response;

static version_magic_number: i32 = 0x5f75e83e; // V0_3
static protocol_magic_number: i32 = 0x7e6970c7; // JSON

pub struct Connection {
    stream: BufferedStream<TcpStream>
}

impl fmt::Show for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "Connection")
    }
}

impl Connection {
    pub fn run(&mut self, term: json::Json) -> RdbResult<Response> {
        use j = serialize::json;
        use std::str;

        let mut global_optargs = box TreeMap::new();
        global_optargs.insert("db".to_owned(), j::List(vec![j::Number(14.),
                                               j::List(vec![j::String("test".to_owned())])]));
        let global_optargs = j::Object(global_optargs);

        let query = j::List(vec![j::Number(1.), term, global_optargs]);

        let res = self.execute_json(query).map(|buf| {
            let str_res = str::from_utf8(buf.as_slice()).unwrap();
            json::from_str(str_res).unwrap()
        });

        let res = res.map_err(|e| response::IoError(e));

        res.and_then(|r| Response::from_json(r))
    }

    fn execute_json(&mut self, json: Json) -> IoResult<Vec<u8>> {
        let json_strbuf = json.to_str().to_strbuf();
        self.execute_raw(json_strbuf.as_bytes())
    }

    fn execute_raw(&mut self, query: &[u8]) -> IoResult<Vec<u8>> {
        let buf: ~[u8] = query.clone().to_owned();
        let token = 666;
        let query_size = buf.len().to_i32().unwrap();

        try!(self.stream.write_le_i64(token));
        try!(self.stream.write_le_i32(query_size));
        try!(self.stream.write(buf));
        try!(self.stream.flush());

        let _recv_token = try!(self.stream.read_le_i64());

        let recv_size = try!(self.stream.read_le_i32());
        self.stream.read_exact(recv_size.to_uint().unwrap())
    }

    // like read_to_end, but stops when a 0 is read.
    fn read_to_null(&mut self) -> IoResult<Vec<u8>> {
        let mut buf = Vec::new();
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
        try!(self.stream.write_le_i32(version_magic_number));
        try!(self.stream.write_le_i32(api_key_len));
        self.stream.write_le_i32(protocol_magic_number)
    }

    fn read_handshake_reply(&mut self) -> IoResult<Vec<u8>> {
        try!(self.stream.flush());
        self.read_to_null()
    }
}

pub fn connect(address: SocketAddr) -> RdbResult<Connection> {
    use self::response::ProtocolError;
    use std::str;

    fn make_conn(address: SocketAddr) -> IoResult<Connection> {
        let stream = try!(TcpStream::connect(address));
        Ok(Connection { stream: BufferedStream::new(stream) })
    }

    fn shake_hands(conn: &mut Connection) -> IoResult<Vec<u8>> {
        try!(conn.write_handshake());
        conn.read_handshake_reply()
    }

    let mut conn = try!(make_conn(address).map_err(|e| { response::IoError(e) }));
    let response = try!(shake_hands(&mut conn).map_err(|e| { response::IoError(e) }));

    match str::from_utf8(response.as_slice()) {
        Some("SUCCESS") => { },
        _ => {
            return Err(ProtocolError("handshake error".to_owned()));
        }
    };
    Ok(conn)
}
