#![feature(phase)]

extern crate serialize;

pub use errors::{Error, RdbResult};
pub use net::{connect, Connection};

mod errors;
mod from_response;
mod net;
pub mod query;
mod term;

#[cfg(test)]
mod test;
