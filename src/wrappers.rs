extern crate private_id_common as common;

use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(a: &str) -> String {
    format!("Hello, {}!", a)
}

#[wasm_bindgen]
pub fn add(a: &str, b: &str) -> String {
    (a.len() + b.len()).to_string()
}

#[wasm_bindgen]
pub fn gen_permute_pattern(n: usize) -> String {
    format!("{:?}", common::permutations::gen_permute_pattern(n))
}

macro_rules! Array {
    ($v:expr) => {$v.into_iter().map(JsValue::from).collect()};
}

#[wasm_bindgen]
pub fn test(a: Vec<u32>) -> Array {
    let b: Vec<usize> = vec![a[0] as usize, a[1] as usize, a[2] as usize];

    let c: Vec<usize> = b;

    let d: Vec<u32> = vec![c[0] as u32, c[1] as u32, c[2] as u32];
    Array!(d)
}

#[allow(dead_code)]
fn usize_to_u64(mut v: Vec<usize>) -> Vec<u64> {
    let ratio = std::mem::size_of::<usize>() / std::mem::size_of::<u64>();
    let length = v.len() * ratio;
    let capacity = v.capacity() * ratio;
    let ptr = v.as_mut_ptr() as *mut u64;
    std::mem::forget(v);

    unsafe { Vec::from_raw_parts(ptr, length, capacity) }
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
pub fn permute(a: Vec<u32>) -> Array {
    let b: Vec<usize> = u32_to_usize(a);

    let mut items: Vec<usize> = b;
    let permutation = vec![2, 1, 0];
    common::permutations::permute(&permutation, &mut items);
    let c: Vec<usize> = items;

    let d: Vec<u32> = usize_to_u32(c);
    Array!(d)
}

// #[wasm_bindgen]
// pub fn permute(items: &Vec<u8>) -> String {
//     // let &mut items: Vec<usize> = a;
//     let permutation = vec![3, 2, 1, 0];//common::permutations::gen_permute_pattern(10);
//     format!("{:?}", common::permutations::permute(&permutation, &mut items))
// }

// #[wasm_bindgen]
// pub fn permute(n: usize, a: [i32; n]) -> String {
//     let &mut items: Vec<usize> = vec!(a);
//     let permutation = common::permutations::gen_permute_pattern(n);
//     format!("{:?}", common::permutations::permute(permutation, items))
// }
