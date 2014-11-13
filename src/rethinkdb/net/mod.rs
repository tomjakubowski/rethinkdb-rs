pub use self::response::{Error, DriverError, ProtocolError, RdbResult,
                         Response, ResponseKind, ResponseAtom,
                         ResponseSequence};

use super::term;

use std::collections::TreeMap;

use serialize::json;
use serialize::json::{Json, ToJson};

use std::fmt;
use std::io::{BufferedStream, IoResult};
use std::io::net::tcp::TcpStream;

mod response;

static VERSION_MAGIC_NUMBER: i32 = 0x5f75e83e; // V0_3
static PROTOCOL_MAGIC_NUMBER: i32 = 0x7e6970c7; // JSON

pub struct Connection {
    stream: BufferedStream<TcpStream>,
    opt_args: OptArgs,
    token: u64
}

struct OptArgs {
    db: Option<String>
}

impl<'a> ToJson for &'a OptArgs {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        if self.db.is_some() {
            d.insert("db".to_string(), (term::Db, (self.db.clone(),)).to_json());
        }
        json::Object(d)
    }
}

impl fmt::Show for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection")
    }
}

pub fn run(conn: &mut Connection, term: Json) -> RdbResult<Response> {
    conn.run(term)
}

impl Connection {
    /// Sets the default database on this connection.
    pub fn use_db<S: StrAllocating>(&mut self, db: S) {
        self.opt_args.db = Some(db.into_string());
    }

    fn run(&mut self, term: Json) -> RdbResult<Response> {
        use std::str;

        const START: u8 = 1;

        let query_type = START;
        let query = (query_type, term, &self.opt_args).to_json();

        let res = self.execute_json(query).map(|buf| {
            let str_res = str::from_utf8(buf.as_slice()).unwrap();
            json::from_str(str_res).unwrap()
        });

        let res = res.map_err(response::IoError); // FIXME: use FromError

        res.and_then(Response::from_json)
    }

    fn execute_json(&mut self, json: Json) -> IoResult<Vec<u8>> {
        self.execute_raw(json.to_string().as_bytes())
    }

    fn execute_raw(&mut self, query: &[u8]) -> IoResult<Vec<u8>> {
        let token = self.token;
        self.token += 1;
        let query_size = query.len();
        assert!(query_size <= ::std::i32::MAX as uint);

        try!(self.stream.write_le_u64(token));
        try!(self.stream.write_le_i32(query_size as i32));
        try!(self.stream.write(query));
        try!(self.stream.flush());

        let _recv_token = try!(self.stream.read_le_u64());

        let recv_size = try!(self.stream.read_le_u32());
        self.stream.read_exact(recv_size as uint)
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
        // FIXME: actually accept an optional authorization key
        let api_key_len = 0;
        try!(self.stream.write_le_i32(VERSION_MAGIC_NUMBER));
        try!(self.stream.write_le_i32(api_key_len));
        self.stream.write_le_i32(PROTOCOL_MAGIC_NUMBER)
    }

    fn read_handshake_reply(&mut self) -> IoResult<Vec<u8>> {
        try!(self.stream.flush());
        self.read_to_null()
    }
}

pub fn connect(host: &str, port: u16) -> RdbResult<Connection> {
    use self::response::ProtocolError;

    fn make_conn(host: &str, port: u16) -> IoResult<Connection> {
        let stream = try!(TcpStream::connect((host, port)));
        Ok(Connection {
            stream: BufferedStream::new(stream),
            opt_args: OptArgs {
                db: None
            },
            token: 0
        })
    }

    let mut conn = try!(make_conn(host, port).map_err(response::IoError)); // FIXME: FromError
    // FIXME: FromError
    try!(conn.write_handshake().map_err(response::IoError));
    // FIXME: FromError
    let response = try!(conn.read_handshake_reply().map_err(response::IoError));

    match response.as_slice() {
        b"SUCCESS" => { },
        _ => {
            return Err(ProtocolError("handshake error".to_string()));
        }
    };
    Ok(conn)
}