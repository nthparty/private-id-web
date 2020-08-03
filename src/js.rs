extern crate private_id_crypto as crypto;

use self::crypto::prelude::Bytes;
use serde_json::json;
use serde_json::from_str;

use compressed_string::ComprString;

pub trait JSON {
    fn to_json(&self) -> String;
    fn from_json(s: String) -> Self;
}

impl JSON for Bytes {
    fn to_json(&self) -> String {
        format!("{}", json!(self))
    }

    fn from_json(s: String) -> Self {
        from_str(&s).unwrap()
    }
}

pub trait JSON_GZ {
    fn to_json_gz(&self) -> String;
    fn from_json_gz(s: String) -> Self;
}

impl JSON_GZ for Bytes {
    fn to_json_gz(&self) -> String {
        let str = &format!("{}", json!(self));
        format!("{}", json!(ComprString::new(str)))
    }

    fn from_json_gz(s: String) -> Self {
        let compr: ComprString = from_str(&s).unwrap();
        from_str(&compr.to_string()).unwrap()
    }
}

pub fn usize_to_u32(mut v: Vec<usize>) -> Vec<u32> {
    let ratio = std::mem::size_of::<usize>() / std::mem::size_of::<u32>();
    let length = v.len() * ratio;
    let capacity = v.capacity() * ratio;
    let ptr = v.as_mut_ptr() as *mut u32;
    std::mem::forget(v);

    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
}

pub fn u32_to_usize(mut v: Vec<u32>) -> Vec<usize> {
    let ratio = std::mem::size_of::<u32>() / std::mem::size_of::<usize>();
    let length = v.len() * ratio;
    let capacity = v.capacity() * ratio;
    let ptr = v.as_mut_ptr() as *mut usize;
    std::mem::forget(v);

    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
}