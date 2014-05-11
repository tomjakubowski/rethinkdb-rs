#![crate_id="restink#0.1.0"]
#![crate_type="lib"]
#![feature(struct_variant)]
#![allow(attribute_usage)]

extern crate collections;
extern crate serialize;

pub use net::{connect};

use serialize::json;
use net::{RdbResult, Response};

pub mod net;
pub mod query;

pub trait Runnable {
    fn to_query(self) -> json::Json;

    fn run(self, conn: &mut net::Connection) -> RdbResult<Response> {
        conn.run(self.to_query())
    }
}

impl Runnable for json::Json {
    fn to_query(self) -> json::Json { self }
}

impl Runnable for query::Table {
    fn to_query(self) -> json::Json { self.term }
}

#[cfg(test)]
mod test;
