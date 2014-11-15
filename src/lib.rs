#![feature(associated_types, macro_rules, phase)]

extern crate serialize;

pub use errors::{Error, RdbResult};
pub use net::{connect, Connection};

mod errors;
mod from_response;
mod net;
pub mod query;
pub mod query2;
mod term;

#[cfg(test)]
mod test;
