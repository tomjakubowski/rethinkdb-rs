#![feature(associated_types, macro_rules, phase)]

extern crate serialize;

pub use net::connect;
pub use net::{Connection, RdbResult, Error};

mod from_response;
mod net;
pub mod query;
pub mod query2;
mod term;

#[cfg(test)]
mod test;
