extern crate rand_core;
extern crate rand;

use rand_core::{CryptoRng, RngCore};
use rand::{thread_rng, Rng};
use self::rand_core::Error;
use std::io::Write;
use self::rand::prelude::ThreadRng;

pub struct CsRng { rng: ThreadRng }

impl CsRng {
    pub fn new() -> CsRng {
        CsRng { rng: rand::thread_rng() }
    }
}

// trait rand_rng {
//     rng
// }

impl CryptoRng for CsRng {}

impl RngCore for CsRng {
    // fn next_u8(&mut self) -> u8 {
    //     8
    // }

    fn next_u32(&mut self) -> u32 {
        self.rng.gen()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.gen()
    }

    fn fill_bytes(&mut self, mut dest: &mut [u8]) {
        let l = dest.len();
        for _ in 0..l {
            let d = [self.rng.gen()];
            dest.write(&d);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
