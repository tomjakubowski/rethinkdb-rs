#![feature(struct_variant)]

extern crate serialize;

#[cfg(test)]
extern crate collections;

use std::io;

use serialize::json;

pub type RdbResult<A> = Result<A, Error>;

#[deriving(Show)]
pub enum Error {
    ClientError(~str),
    CompileError(~str),
    RuntimeError(~str),
    ProtocolError(~str),
    IoError(io::IoError)
}

impl Error {
    pub fn new(kind: int, res: json::Json) -> Error {
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
    ResponseComplete,
    ResponsePartial
}

#[deriving(Show)]
struct RawResponse {
    res_type: int,
    res: json::Json
}

impl RawResponse {
    fn from_json(json: json::Json) -> RdbResult<RawResponse> {
        let values = (json.find(&"t".to_owned()).and_then(|x| x.as_number()),
                      json.find(&"r".to_owned()));

        match values {
            (Some(t), Some(r)) => Ok(RawResponse { res_type: t as int, res: r.clone() }),
            _ => {
                let msg = format!("JSON decoding error: {}", json);
                return Err(ProtocolError(msg));
            }
        }
    }
}

#[deriving(Show)]
pub struct Response {
    kind: ResponseKind,
    values: json::Json
}

impl Response {
    pub fn from_json(json: json::Json) -> RdbResult<Response> {
        RawResponse::from_json(json).and_then(|raw: RawResponse| {
            match raw.res_type {
                1 | 2 => Ok(Response::new(ResponseComplete, raw.res)),
                3 => Ok(Response::new(ResponsePartial, raw.res)),
                n => Err(Error::new(n, raw.res))
            }
        })
    }

    fn new(kind: ResponseKind, res: json::Json) -> Response {
        Response {
            kind: kind,
            values: res
        }
    }
}

#[cfg(test)]
mod test {
    use serialize::json;

    use super::{RawResponse, Response, ResponseComplete};

    #[test]
    fn test_raw_response_from_json() {
        let json = json::from_str(r#"{"t": 1, "r": [["bar","foo"]]}"#).unwrap();
        let raw_res = RawResponse::from_json(json).unwrap();
        let tables = json::List(vec![json::String("bar".to_owned()),
                                     json::String("foo".to_owned())]);

        assert_eq!(raw_res.res_type, 1);
        assert_eq!(raw_res.res, json::List(vec![tables]));
    }

    #[test]
    fn test_success_from_json() {
        let json = json::from_str(r#"{"t": 1, "r": [["bar","foo"]]}"#).unwrap();
        let res = Response::from_json(json).unwrap();
        let tables = json::List(vec![json::String("bar".to_owned()),
                                     json::String("foo".to_owned())]);

        let Response { kind: kind, values: values } = res;

        assert_eq!(kind, ResponseComplete);
        assert_eq!(values, json::List(vec![tables]));
    }

}
