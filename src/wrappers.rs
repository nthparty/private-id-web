extern crate private_id_common as common;

use js_sys::Array;
use wasm_bindgen::prelude::*;

macro_rules! Array {
    ($v:expr) => {$v.into_iter().map(JsValue::from).collect()};
}

fn usize_to_u32(mut v: Vec<usize>) -> Vec<u32> {
    let ratio = std::mem::size_of::<usize>() / std::mem::size_of::<u32>();
    let length = v.len() * ratio;
    let capacity = v.capacity() * ratio;
    let ptr = v.as_mut_ptr() as *mut u32;
    std::mem::forget(v);

    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
}

fn u32_to_usize(mut v: Vec<u32>) -> Vec<usize> {
    let ratio = std::mem::size_of::<u32>() / std::mem::size_of::<usize>();
    let length = v.len() * ratio;
    let capacity = v.capacity() * ratio;
    let ptr = v.as_mut_ptr() as *mut usize;
    std::mem::forget(v);

    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
}

#[wasm_bindgen]
pub fn permute(p: Vec<u32>, a: Vec<u32>) -> Array {
    let mut items: Vec<usize> = u32_to_usize(a);
    let permutation = u32_to_usize(p);

    common::permutations::permute(&permutation, &mut items);

    Array!(usize_to_u32(items))
}
