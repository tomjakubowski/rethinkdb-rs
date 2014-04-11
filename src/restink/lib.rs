#![crate_id="restink#0.1.0"]
#![crate_type="lib"]

extern crate collections;
pub use connect = net::connect;

// this unfortunately interacts poorly with flycheck. maybe patch
// flycheck to allow a buffer-local override of the file to check with
// rustc --no-trans (e.g. the file containing the crate root)?
// #![feature(phase)] #[phase(syntax, link)]
// extern crate log;

mod net;
mod protocol;

#[cfg(test)]
mod test;
