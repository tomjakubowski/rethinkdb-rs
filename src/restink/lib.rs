#![crate_id="restink#0.1.0"]
#![crate_type="lib"]
#![feature(struct_variant)]
#![allow(attribute_usage)]

extern crate collections;
extern crate serialize;

pub use net::{connect};
use net::{RdbResult, Response};

use serialize::json;
use serialize::json::ToJson;

pub mod net;
pub mod query;

#[cfg(test)]
mod test;

pub trait Runnable : ToJson {
    fn run(self, conn: &mut net::Connection) -> RdbResult<Response> {
        conn.run(self.to_json())
    }
}

impl Runnable for json::Json { }
impl Runnable for query::Table { }
