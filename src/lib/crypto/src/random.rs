extern crate rand_core;

use rand_core::{CryptoRng, RngCore};
use self::rand_core::Error;
use std::io::Write;
use std::borrow::BorrowMut;

pub struct CsRng;

impl CryptoRng for CsRng {}

impl RngCore for CsRng {
    fn next_u32(&mut self) -> u32 {
        32
    }

    fn next_u64(&mut self) -> u64 {
        64
    }

    fn fill_bytes(&mut self, mut dest: &mut [u8]) {
        let l = dest.len();
        let d = [1u8; 64];
        dest.write(&d);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(())
    }
}
