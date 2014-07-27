#![crate_name = "restink"]

extern crate serialize;

pub use net::connect;
pub use net::{Connection, RdbResult, Error};

mod from_response;
mod net;
pub mod query;
mod term;

#[cfg(test)]
mod test;
