#![crate_id="restink#0.1.0"]
#![crate_type="lib"]

extern crate collections;
pub use net::{connect, Connection};

mod net;

#[cfg(test)]
mod test;
