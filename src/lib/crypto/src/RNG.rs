//
//
// mod imp {
//     use rdrand::RdRand;
//     use std::io;
//     use rand_core::RngCore;
//
//     pub struct OsRng{
//         gen: RdRand
//     }
//
//     impl OsRng {
//         pub fn new() -> io::Result<OsRng> {
//             match RdRand::new() {
//                 Ok(rng) => Ok(OsRng { gen: rng }),
//                 Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Not supported"))
//             }
//         }
//
//         pub(crate) fn next_u32(&mut self) -> u32 {
//             match self.gen.try_next_u32() {
//                 Some(n) => n,
//                 None => panic!("Non-recoverable hardware failure has occured")
//             }
//         }
//
//         pub(crate) fn next_u64(&mut self) -> u64 {
//             match self.gen.try_next_u64() {
//                 Some(n) => n,
//                 None => panic!("Non-recoverable hardware failure has occured")
//             }
//         }
//
//         pub(crate) fn fill_bytes(&mut self, v: &mut [u8]) {
//             match self.gen.try_fill_bytes(v) {
//                 Ok(_) => {},
//                 Err(_) => panic!("Non-recoverable hardware failure has occured")
//             }
//         }
//     }
// }
//
// use rand_core::OsRng;
//
// use rand_core::{CryptoRng, RngCore, Error, impls};
//
// #[derive(Clone)]
// pub struct RNG(imp::OsRng);
//
// impl RNG {
//     /// Create a new `RNG`.
//     pub fn new() -> Result<RNG, Error> {
//         imp::OsRng::new().map(RNG)
//     }
// }
//
// impl CryptoRng for RNG {}
//
// impl RngCore for RNG {
//     fn next_u32(&mut self) -> u32 {
//         impls::next_u32_via_fill(self)
//     }
//
//     fn next_u64(&mut self) -> u64 {
//         impls::next_u64_via_fill(self)
//     }
//
//     fn fill_bytes(&mut self, dest: &mut [u8]) {
//         use std::{time, thread};
//
//         // We cannot return Err(..), so we try to handle before panicking.
//         const MAX_RETRY_PERIOD: u32 = 10; // max 10s
//         const WAIT_DUR_MS: u32 = 100; // retry every 100ms
//         let wait_dur = time::Duration::from_millis(WAIT_DUR_MS as u64);
//         const RETRY_LIMIT: u32 = (MAX_RETRY_PERIOD * 1000) / WAIT_DUR_MS;
//         const TRANSIENT_RETRIES: u32 = 8;
//         let mut err_count = 0;
//         let mut error_logged = false;
//
//         // Maybe block until the OS RNG is initialized
//         let mut read = 0;
//         if let Ok(n) = self.0.test_initialized(dest, true) { read = n };
//         let dest = &mut dest[read..];
//
//         loop {
//             if let Err(e) = self.try_fill_bytes(dest) {
//                 if err_count >= RETRY_LIMIT {
//                 }
//
//                 if e.kind.should_wait() {
//                     if !error_logged {
//                         error_logged = true;
//                     }
//                     err_count += 1;
//                     thread::sleep(wait_dur);
//                     continue;
//                 }
//             }
//
//             break;
//         }
//     }
//
//     fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
//         // Some systems do not support reading 0 random bytes.
//         // (And why waste a system call?)
//         if dest.len() == 0 { return Ok(()); }
//
//         let read = self.0.test_initialized(dest, false)?;
//         let dest = &mut dest[read..];
//
//         let max = self.0.max_chunk_size();
//         if dest.len() <= max {
//         } else {
//         }
//         for slice in dest.chunks_mut(max) {
//             self.0.fill_chunk(slice)?;
//         }
//         Ok(())
//     }
// }