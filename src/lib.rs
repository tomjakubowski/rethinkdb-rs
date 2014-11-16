#![feature(associated_types, if_let, macro_rules, phase, struct_variant)]

extern crate serialize;

pub use errors::{Error, RdbResult};
pub use net::{connect, Connection};
pub use query::Query;

mod errors;
mod from_response;
mod net;
pub mod query;

#[cfg(test)]
mod test;
