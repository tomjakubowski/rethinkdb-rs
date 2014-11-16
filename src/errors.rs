use std::error::FromError;
use std::io;
use serialize::json::{mod, Json};

const CLIENT_ERROR: u8 = 16;
const COMPILE_ERROR: u8 = 17;
const RUNTIME_ERROR: u8 = 18;

pub type RdbResult<A> = Result<A, Error>;

// FIXME: impl FromError<IoError> et al
// FIXME: this should live somewhere else
#[deriving(Show)]
pub enum Error {
    // FIXME: JSON decoding error + the like?
    ClientError(String),
    CompileError(String),
    RuntimeError(String),
    ProtocolError(String),
    DriverError(String), // FIXME: should this be merged with ProtocolError?
    JsonParseError(json::ParserError),
    IoError(io::IoError)
}

impl Error {
    pub fn from_code_res(code: u8, res: Json) -> Error {
        let msgs = res.as_list();
        let msg = match msgs.map(|x| x.as_slice()) {
            Some([json::String(ref x)]) => x.to_string(),
            _ => {
                return ProtocolError(format!("couldn't find error message in {}", res));
            }
        };

        match code {
            CLIENT_ERROR => ClientError(msg),
            COMPILE_ERROR => CompileError(msg),
            RUNTIME_ERROR => RuntimeError(msg),
            _ => ProtocolError(format!("unrecognized error number: {}", code))
        }
    }
}

impl FromError<io::IoError> for Error {
    fn from_error(e: io::IoError) -> Error { IoError(e) }
}

impl FromError<json::ParserError> for Error {
    fn from_error(e: json::ParserError) -> Error { JsonParseError(e) }
}
