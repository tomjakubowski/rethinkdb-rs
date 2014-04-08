#![crate_id="restink#0.1.0"]
#![crate_type="lib"]

// #![feature(globs)]
// extern crate protobuf;

pub use net::{Connection, connect};

// #[allow(non_camel_case_types, dead_code, uppercase_variables)]
// mod ql2;

mod net;

#[cfg(test)]
mod test;
