#![crate_id="restink#0.1.0"]
#![crate_type="lib"]

extern crate collections;
extern crate serialize;

pub use net::{connect, Connection};

mod net;
pub mod query;

#[cfg(test)]
mod test;
