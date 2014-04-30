#![crate_id="restink#0.1.0"]
#![crate_type="lib"]
#![feature(struct_variant)]
#![allow(attribute_usage)]

extern crate collections;
extern crate serialize;

pub use net::{connect, Connection, Response};

mod net;
pub mod query;

#[cfg(test)]
mod test;
