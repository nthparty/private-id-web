#![forbid(unsafe_code)]
#![crate_name = "common"]

#[macro_use]
extern crate log;

/// Simple file io
pub mod files;

/// Collections utils
pub mod vectors;

/// Permutation utils
pub mod permutations;
