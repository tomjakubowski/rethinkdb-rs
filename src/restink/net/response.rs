#![feature(struct_variant)]

extern crate serialize;

#[cfg(test)]
extern crate collections;

use serialize::json;

#[deriving(Show, Eq)]
pub enum SuccessKind {
    SuccessComplete,
    SuccessPartial
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

impl RawResponse {
    pub fn from_json(json: json::Json) -> Option<RawResponse> {
        let mut map: json::Object = match json.as_object() {
            None => { return None; },
            Some(x) => x.clone()
        };

        let values = (map.pop(&"t".to_owned()).and_then(|x| x.as_number()),
                      map.pop(&"r".to_owned()));
        match values {
            (Some(t), Some(r)) => {
                Some(RawResponse {
                    res_type: t as int,
                    res: r
                })
            },
            _ => None
        }
    }
}

#[deriving(Show)]
pub enum Response {
    Error { kind: ErrorKind, message: ~str },
    Success { kind: SuccessKind, values: json::Json }
}

impl Response {
    pub fn from_json(json: json::Json) -> Option<Response> {
        RawResponse::from_json(json).and_then(|raw: RawResponse| {
            Some(match raw.res_type {
                1 | 2 => {
                    Response::new_success(SuccessComplete, raw.res)
                },
                3 => {
                    Response::new_success(SuccessPartial, raw.res)
                },
                16 => {
                    Response::new_error(ClientError, raw.res)
                },
                17 => {
                    Response::new_error(CompileError, raw.res)
                },
                18 => {
                    Response::new_error(RuntimeError, raw.res)
                }
                _ => { return None }
            })
        })
    }

    fn new_error(kind: ErrorKind, res: json::Json) -> Response {
        let msgs = res.as_list().expect(format!("couldn't get error message from {}", res));
        let msg = match msgs.as_slice() {
            [json::String(ref x)] => { x.to_owned() },
            _ => fail!(format!("couldn't get error message from {}", res))
        };
        Error {
            kind: kind,
            message: msg
        }
    }

    fn new_success(kind: SuccessKind, res: json::Json) -> Response {
        Success {
            kind: kind,
            values: res
        }
    }
}

#[cfg(test)]
mod test {
    use serialize::json;

    use super::{RawResponse, Response, Success, SuccessComplete};

    #[test]
    fn test_raw_response_from_json() {
        let json = json::from_str(r#"{"t": 1, "r": [["bar","foo"]]}"#).unwrap();
        let raw_res = RawResponse::from_json(json).unwrap();
        let tables = json::List(~[json::String(~"bar"), json::String(~"foo")]);

        assert_eq!(raw_res.res_type, 1);
        assert_eq!(raw_res.res, json::List(~[tables]));
    }

    #[test]
    fn test_success_from_json() {
        let json = json::from_str(r#"{"t": 1, "r": [["bar","foo"]]}"#).unwrap();
        let res = Response::from_json(json).unwrap();
        let tables = json::List(~[json::String(~"bar"), json::String(~"foo")]);

        let (kind, values) = match res {
            Success { kind: kind, values: values } => (kind, values),
            _ => { fail!("the test is bad!") }
        };

        assert_eq!(kind, SuccessComplete);
        assert_eq!(values, json::List(~[tables]));
    }
}
