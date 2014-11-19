use serialize::json::Json;

use errors::RdbResult;
use errors::Error::{mod, ProtocolError};

#[deriving(Show, PartialEq, Eq)]
pub enum ResponseKind {
    Atom,
    Sequence,
    Partial
}

#[deriving(Show)]
struct RawResponse {
    res_type: u8,
    res: Json
}

impl RawResponse {
    fn from_json(json: Json) -> RdbResult<RawResponse> {
        let values = (json.find("t").and_then(|x| x.as_u64()),
                      json.find("r"));

        match values {
            (Some(t), Some(r)) => {
                assert!(t <= ::std::u8::MAX as u64);
                // FIXME: clone
                Ok(RawResponse { res_type: t as u8, res: r.clone() })
            },
            _ => {
                let msg = format!("Couldn't unpack response: {}", json);
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

const SUCCESS_ATOM: u8 = 1;
const SUCCESS_SEQUENCE: u8 = 2;
const SUCCESS_PARTIAL: u8 = 3;

impl Response {
    pub fn from_json(json: Json) -> RdbResult<Response> {
        RawResponse::from_json(json).and_then(|raw: RawResponse| {
            match raw.res_type {
                SUCCESS_ATOM => Ok(Response::new(ResponseKind::Atom, raw.res)),
                SUCCESS_SEQUENCE => Ok(Response::new(ResponseKind::Sequence, raw.res)),
                SUCCESS_PARTIAL => Ok(Response::new(ResponseKind::Partial, raw.res)),
                n => Err(Error::from_code_res(n, raw.res))
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

    use super::{RawResponse, Response, ResponseKind};

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

        let Response { kind, values } = res;

        assert_eq!(kind, ResponseKind::Atom);
        assert_eq!(values, json::List(vec![tables]));
    }

}
