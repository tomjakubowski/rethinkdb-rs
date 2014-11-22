#![feature(associated_types, if_let, macro_rules, phase)]
#![experimental]

#[phase(plugin, link)] extern crate log;
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
