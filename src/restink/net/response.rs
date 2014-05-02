#![feature(struct_variant)]

extern crate serialize;

#[cfg(test)]
extern crate collections;

use serialize::json;
// use serialize::{Decodable, Decoder};

pub type RdbResult<A> = Result<A, RdbError>;

#[deriving(Show, Eq)]
pub enum ResponseKind {
    ResponseComplete,
    ResponsePartial
}

#[deriving(Show, Eq)]
pub enum ErrorKind {
    ClientError,
    CompileError,
    RuntimeError
}

#[deriving(Show)]
struct RawResponse {
    res_type: int,
    res: json::Json
}

// argh, could have done this instead
// impl Decodable<json::Decoder, json::Error> for RawResponse {
// }

impl RawResponse {
    pub fn from_json(json: json::Json) -> RdbResult<RawResponse> {
        let mut map: json::Object = match json.as_object() {
            None => fail!("FIXME"),
            Some(x) => x.clone()
        };

        let values = (map.pop(&"t".to_owned()).and_then(|x| x.as_number()),
                      map.pop(&"r".to_owned()));
        match values {
            (Some(t), Some(r)) => {
                Ok(RawResponse {
                    res_type: t as int,
                    res: r
                })
            },
            _ => fail!("FIXME")
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
                1 | 2 => {
                    Ok(Response::new(ResponseComplete, raw.res))
                },
                3 => {
                    Ok(Response::new(ResponsePartial, raw.res))
                },
                16 => {
                    Err(new_error(ClientError, raw.res))
                },
                17 => {
                    Err(new_error(CompileError, raw.res))
                },
                18 => {
                    Err(new_error(RuntimeError, raw.res))
                }
                _ => { fail!("FIXME") }
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

#[deriving(Show)]
pub struct RdbError {
    kind: ErrorKind,
    desc: ~str
}

fn new_error(kind: ErrorKind, res: json::Json) -> RdbError {
    // todo: remove failures, instead return an RdbError
    let msgs = res.as_list().expect(format!("couldn't get error message from {}", res));
    let msg = match msgs.as_slice() {
        [json::String(ref x)] => { x.to_owned() },
        _ => fail!(format!("couldn't get error message from {}", res))
    };
    RdbError {
        kind: kind,
        desc: msg
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
        let tables = json::List(box [json::String("bar".to_owned()),
                                     json::String("foo".to_owned())]);

        assert_eq!(raw_res.res_type, 1);
        assert_eq!(raw_res.res, json::List(box [tables]));
    }

    #[test]
    fn test_success_from_json() {
        let json = json::from_str(r#"{"t": 1, "r": [["bar","foo"]]}"#).unwrap();
        let res = Response::from_json(json).unwrap();
        let tables = json::List(box [json::String("bar".to_owned()),
                                     json::String("foo".to_owned())]);

        let Response { kind: kind, values: values } = res;

        assert_eq!(kind, ResponseComplete);
        assert_eq!(values, json::List(box [tables]));
    }
}
