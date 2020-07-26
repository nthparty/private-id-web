extern crate rand_core;

use rand_core::{CryptoRng, RngCore};
use self::rand_core::Error;
use rand::AsByteSliceMut;
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
        // // dest.as_mut() = [dest.len(); u8];
        // // let dest = &mut dest[read..];
        // let mut dest = &mut dest[0..];

        let l = dest.len();

        let d = [1u8, 2u8, 3u8, 4u8, 5u8];
        // let c: &[u8] = &d;
        dest.write(&d);

        let m = 1+1;
        let mm = m+m;
        // let read: [u8; 64] = [0; 64];
        let mut read: &mut [u8] = &mut [];
        // read.write(&mut [64; 8]);
        // read.write(&[6u8, 6u8, 6u8][0..2]);
        // read = &[6u8, 6u8, 6u8][0..2];
        // read[0] = 7;


        // let mut xs: [u8; 64] = [6; 64];
        // let x: &mut [u8] = &mut xs;


        // let read = &mut dest[0..];
        // unsafe {
        // dest = (&mut [6; 64]).to_owned();
        // let a: [u8] = *[6; 64];
        // *dest = a;
        // let b = [6u8; 64];
        // *dest = b.clone()[0..64];
        // dest.write_u8(6);
        // }
        // dest = &mut dest[read..]
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(())
    }
}
