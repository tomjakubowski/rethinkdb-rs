#![crate_id = "restink#0.1.0"]
#![crate_type = "lib"]
#![allow(attribute_usage)]

extern crate collections;
extern crate serialize;

pub use net::connect;
pub use net::{Connection, RdbResult, Error};

use net::Response;

use serialize::json;
use serialize::json::ToJson;

mod net;
pub mod query;

#[cfg(test)]
mod test;

trait FromResponse {
    fn from_response(res: Response) -> RdbResult<Self>;
}

// someday... https://github.com/rust-lang/rfcs/pull/48
// impl<T: Decodable<json::Decoder, json::DecoderError>> FromResponse for T {
//     fn from_response(res: Response) -> RdbResult<T> {
//         use net::{ResponseAtom, ResponseSequence};

//         let val = match res.kind {
//             ResponseAtom => {
//                 let list = res.values.as_list().unwrap();
//                 list.get(0).clone()
//             },
//             ResponseSequence => {
//                 res.values
//             },
//             _ => {
//                 unimplemented!()
//             }
//         };
//         let mut decoder = json::Decoder::new(val);
//         let decoded: T = Decodable::decode(&mut decoder).unwrap();
//         Ok(decoded)
//     }
// }

impl FromResponse for Vec<StrBuf> {
    fn from_response(res: Response) -> RdbResult<Vec<StrBuf>> {
        use net::{DriverError, ResponseAtom, ResponseSequence};

        match res.kind {
            ResponseAtom => {
                // vvv bad
                let list = res.values.as_list().unwrap();
                let list = list.get(0).as_list().unwrap();

                Ok(list.iter().map(|s| {
                    s.as_string().unwrap().to_strbuf()
                }).collect())
            },
            ResponseSequence => {
                Err(DriverError(format!("FIXME ResponseSequence {}", res)))
            },
            _ => {
                Err(DriverError(format!("Couldn't convert {} to Vec<StrBuf>", res)))
            }
        }
    }
}

impl FromResponse for query::Writes {
    // vvvv this is all very very bad
    fn from_response(res: Response) -> RdbResult<query::Writes> {
        use serialize::Decodable;
        let list = res.values.as_list().unwrap();
        let mut decoder = json::Decoder::new(list.get(0).clone()); // FIXME
        let insertion: query::Writes = Decodable::decode(&mut decoder).unwrap(); // FIXME
        Ok(insertion)
    }
}

impl FromResponse for Response {
    fn from_response(res: Response) -> RdbResult<Response> { Ok(res) }
}

impl FromResponse for json::Json {
    fn from_response(res: Response) -> RdbResult<json::Json> { Ok(res.values) }
}

impl FromResponse for () {
    fn from_response(_: Response) -> RdbResult<()> { Ok(()) }
}

impl<Out: FromResponse> query::Func<Out> {
    pub fn run(self, conn: &mut Connection) -> RdbResult<Out> {
        net::run(conn, self.to_json()).and_then(|res| { FromResponse::from_response(res) })
    }
}
