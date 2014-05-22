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

pub trait Runnable<Out: FromResponse> : ToJson {
    fn run(self, conn: &mut net::Connection) -> RdbResult<Out> {
        conn.run(self.to_json()).and_then(|res| { FromResponse::from_response(res) })
    }
}

impl Runnable<Response> for query::Table {}
impl Runnable<Response> for query::Func<json::Json> {}
impl Runnable<query::Writes> for query::Func<query::Writes> {}
