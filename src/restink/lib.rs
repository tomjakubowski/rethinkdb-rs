#![crate_id = "restink#0.1.0"]
#![crate_type = "lib"]
#![allow(attribute_usage)]
#![feature(default_type_params)]

extern crate collections;
extern crate serialize;

pub use net::{connect};
use net::{RdbResult, Response};

use serialize::Decodable;
use serialize::json;
use serialize::json::ToJson;

pub mod net;
pub mod query;

#[cfg(test)]
mod test;

trait FromResponse {
    fn from_response(res: Response) -> RdbResult<Self>;
}

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

pub trait Runnable<Out: FromResponse> : ToJson {
    fn run(self, conn: &mut net::Connection) -> RdbResult<Out> {
        conn.run(self.to_json()).and_then(|res| { FromResponse::from_response(res) })
    }
}

// FIXME: query::Table -> query::TableQuery
impl Runnable<Response> for query::Table {}
impl<T: FromResponse> Runnable<T> for query::Func<T> {}
