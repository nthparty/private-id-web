
// MIT License

extern crate csv;

use common::{permutations::gen_permute_pattern};

use std::sync::{Arc, RwLock};

use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ProtocolError {
    ErrorDeserialization(String),
    ErrorSerialization(String),
    ErrorEncryption(String),
    ErrorCalcSetDiff(String),
    ErrorReencryption(String),
    ErrorIO(String),
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "protocol error {}", self)
    }
}

impl Error for ProtocolError {}

fn fill_permute(permutation: Arc<RwLock<Vec<usize>>>, text_len: usize) {
    if let Ok(mut wguard) = permutation.write() {
        if wguard.is_empty() {
            let mut pm = gen_permute_pattern(text_len);
            wguard.append(&mut pm);
        }
    }
}

pub mod company;
pub mod partner;
pub mod traits;
