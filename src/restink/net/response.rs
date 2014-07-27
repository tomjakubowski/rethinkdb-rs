extern crate serialize;

use std::io;

use serialize::json;
use serialize::json::Json;

pub type RdbResult<A> = Result<A, Error>;

#[deriving(Show)]
pub enum Error {
    ClientError(String),
    CompileError(String),
    RuntimeError(String),
    ProtocolError(String),
    DriverError(String), // FIXME: should this be merged with ProtocolError?
    IoError(io::IoError)
}

static CLIENT_ERROR: u8 = 16;
static COMPILE_ERROR: u8 = 17;
static RUNTIME_ERROR: u8 = 18;

impl Error {
    pub fn new(kind: u8, res: Json) -> Error {
        let msgs = res.as_list();
        let msg = match msgs.map(|x| x.as_slice()) {
            Some([json::String(ref x)]) => x.to_string(),
            _ => {
                return ProtocolError(format!("couldn't find error message in {}", res));
            }
        };

        match kind {
            CLIENT_ERROR => ClientError(msg),
            COMPILE_ERROR => CompileError(msg),
            RUNTIME_ERROR => RuntimeError(msg),
            _ => ProtocolError(format!("unrecognized error number: {}", kind))
        }
    }
}

#[deriving(Show, PartialEq, Eq)]
pub enum ResponseKind {
    ResponseAtom,
    ResponseSequence,
    ResponsePartial
}

#[deriving(Show)]
struct RawResponse {
    res_type: u8,
    res: Json
}

impl RawResponse {
    fn from_json(json: Json) -> RdbResult<RawResponse> {
        let values = (json.find(&"t".to_string()).and_then(|x| x.as_number()),
                      json.find(&"r".to_string()));

        match values {
            (Some(t), Some(r)) => {
                Ok(RawResponse { res_type: t as u8, res: r.clone() })
            },
            _ => {
                let msg = format!("JSON decoding error: {}", json);
                Err(ProtocolError(msg))
            }
        }
    }
}

#[deriving(Show)]
pub struct Response {
    pub kind: ResponseKind,
    pub values: Json
}

static SUCCESS_ATOM: u8 = 1;
static SUCCESS_SEQUENCE: u8 = 2;
static SUCCESS_PARTIAL: u8 = 3;

impl Response {
    pub fn from_json(json: Json) -> RdbResult<Response> {
        RawResponse::from_json(json).and_then(|raw: RawResponse| {
            match raw.res_type {
                SUCCESS_ATOM => Ok(Response::new(ResponseAtom, raw.res)),
                SUCCESS_SEQUENCE => Ok(Response::new(ResponseSequence, raw.res)),
                SUCCESS_PARTIAL => Ok(Response::new(ResponsePartial, raw.res)),
                n => Err(Error::new(n, raw.res))
            }
        })
    }

    fn new(kind: ResponseKind, res: Json) -> Response {
        Response {
            kind: kind,
            values: res
        }
    }
}

#[cfg(test)]
mod test {
    use serialize::json;

    use super::{RawResponse, Response, ResponseAtom};

    #[test]
    fn test_raw_response_from_json() {
        let json = json::from_str(r#"{"t": 1, "r": [["bar","foo"]]}"#).unwrap();
        let raw_res = RawResponse::from_json(json).unwrap();
        let tables = json::List(vec![json::String("bar".to_string()),
                                     json::String("foo".to_string())]);

        assert_eq!(raw_res.res_type, 1);
        assert_eq!(raw_res.res, json::List(vec![tables]));
    }

    #[test]
    fn test_success_from_json() {
        let json = json::from_str(r#"{"t": 1, "r": [["bar","foo"]]}"#).unwrap();
        let res = Response::from_json(json).unwrap();
        let tables = json::List(vec![json::String("bar".to_string()),
                                     json::String("foo".to_string())]);

        let Response { kind: kind, values: values } = res;

        assert_eq!(kind, ResponseAtom);
        assert_eq!(values, json::List(vec![tables]));
    }

}
