
// MIT License

#![forbid(unsafe_code)]
#![crate_name = "rpc"]

#[macro_use]
extern crate log;

extern crate crypto;
extern crate prost;
extern crate tonic;

pub mod connect;
pub mod proto;
