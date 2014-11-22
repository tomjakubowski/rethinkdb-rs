use std::error::FromError;
use std::io;
use serialize::json::{mod, Json};

const CLIENT_ERROR: u8 = 16;
const COMPILE_ERROR: u8 = 17;
const RUNTIME_ERROR: u8 = 18;

pub type RdbResult<A> = Result<A, Error>;

#[deriving(Show)]
pub enum Error {
    ClientError(String),
    CompileError(String),
    RuntimeError(String),
    DriverError(String),
    JsonParseError(json::ParserError),
    IoError(io::IoError)
}

impl Error {
    pub fn from_code_res(code: u8, res: Json) -> Error {
        use Error::{ClientError, CompileError, RuntimeError, DriverError};
        let msgs = res.as_list();
        let msg = match msgs.map(|x| x.as_slice()) {
            Some([json::String(ref x)]) => x.to_string(),
            _ => return DriverError(format!("couldn't find error message in {}", res))
        };

        match code {
            CLIENT_ERROR => ClientError(msg),
            COMPILE_ERROR => CompileError(msg),
            RUNTIME_ERROR => RuntimeError(msg),
            _ => DriverError(format!("unrecognized error number: {}", code))
        }
    }
}

impl FromError<io::IoError> for Error {
    fn from_error(e: io::IoError) -> Error { Error::IoError(e) }
}

impl FromError<json::ParserError> for Error {
    fn from_error(e: json::ParserError) -> Error { Error::JsonParseError(e) }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        use Error::{ClientError, CompileError, RuntimeError, DriverError};
        use Error::{JsonParseError, IoError};
        match *self {
            ClientError(..) => "RethinkDB client error",
            CompileError(..) => "RethinkDB compile error",
            RuntimeError(..) => "RethinkDB runtime error",
            DriverError(..) => "RethinkDB driver error",
            JsonParseError(..) => "RethinkDB JSON error",
            IoError(ref io_err) => io_err.description()
        }
    }

    fn detail(&self) -> Option<String> {
        use Error::{ClientError, CompileError, RuntimeError, DriverError};
        use Error::{JsonParseError, IoError};
        match *self {
            ClientError(ref s) => Some(s.clone()),
            CompileError(ref s) => Some(s.clone()),
            RuntimeError(ref s) => Some(s.clone()),
            DriverError(ref s) => Some(s.clone()),
            JsonParseError(ref err) => {
                Some(format!("{}", err))
            },
            IoError(ref io_err) => io_err.detail()
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        use std::error::Error;
        use Error::IoError;
        match *self {
            IoError(ref io_err) => Some(io_err as &Error),
            _ => None
        }
    }
}
