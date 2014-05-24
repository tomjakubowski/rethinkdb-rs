extern crate serialize;

#[cfg(test)]
extern crate collections;

use std::io;

use serialize::json;
use serialize::json::Json;

pub type RdbResult<A> = Result<A, Error>;

#[deriving(Show)]
pub enum Error {
    ClientError(~str),
    CompileError(~str),
    RuntimeError(~str),
    ProtocolError(~str),
    DriverError(~str), // FIXME: should this be merged with ProtocolError?
    IoError(io::IoError)
}

impl Error {
    pub fn new(kind: int, res: Json) -> Error {
        let msgs = res.as_list();
        let msg = match msgs.map(|x| x.as_slice()) {
            Some([json::String(ref x)]) => x.to_owned(),
            _ => {
                return ProtocolError(format!("couldn't find error message in {}", res));
            }
        };

        match kind {
            16 => ClientError(msg),
            17 => CompileError(msg),
            18 => RuntimeError(msg),
            _ => ProtocolError(format!("unrecognized error number: {}", kind))
        }
    }
}

#[deriving(Show, Eq)]
pub enum ResponseKind {
    ResponseAtom,
    ResponseSequence,
    ResponsePartial
}

#[deriving(Show)]
struct RawResponse {
    res_type: int,
    res: Json
}

impl RawResponse {
    fn from_json(json: Json) -> RdbResult<RawResponse> {
        let values = (json.find(&"t".to_strbuf()).and_then(|x| x.as_number()),
                      json.find(&"r".to_strbuf()));

        match values {
            (Some(t), Some(r)) => {
                Ok(RawResponse { res_type: t as int, res: r.clone() })
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

impl Response {
    pub fn from_json(json: Json) -> RdbResult<Response> {
        RawResponse::from_json(json).and_then(|raw: RawResponse| {
            match raw.res_type {
                1 => Ok(Response::new(ResponseAtom, raw.res)),
                2 => Ok(Response::new(ResponseSequence, raw.res)),
                3 => Ok(Response::new(ResponsePartial, raw.res)),
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
        let tables = json::List(vec![json::String("bar".to_strbuf()),
                                     json::String("foo".to_strbuf())]);

        assert_eq!(raw_res.res_type, 1);
        assert_eq!(raw_res.res, json::List(vec![tables]));
    }

    #[test]
    fn test_success_from_json() {
        let json = json::from_str(r#"{"t": 1, "r": [["bar","foo"]]}"#).unwrap();
        let res = Response::from_json(json).unwrap();
        let tables = json::List(vec![json::String("bar".to_strbuf()),
                                     json::String("foo".to_strbuf())]);

        let Response { kind: kind, values: values } = res;

        assert_eq!(kind, ResponseAtom);
        assert_eq!(values, json::List(vec![tables]));
    }

}
