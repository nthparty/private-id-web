extern crate private_id_common as common;
extern crate private_id_crypto as crypto;
extern crate private_id_protocol as protocol;
// extern crate private_id_protocol_rpc as protocol_rpc;

use wasm_bindgen::prelude::*;
use super::js::*;
use serde_json::json;

#[wasm_bindgen]
pub fn test() -> String {
    // let &mut input_with_headers;
    // let var_name = protocol::private_id::partner::PartnerPrivateId::load_data("", input_with_headers);
    // protocol::private_id::partner::PartnerPrivateId::new();
    let hello = "Hello".to_string();

    format!("{}", json!(
        vec!([
            vec!([hello.clone(), hello.clone(), hello.clone()]),
            vec!([hello.clone(), hello.clone(), hello.clone()])
        ])
    ))
}

#[wasm_bindgen]
pub fn permute(p: Vec<u32>, a: Vec<u32>) -> Vec<u32> {
    let mut items: Vec<usize> = u32_to_usize(a);
    let permutation = u32_to_usize(p);

    common::permutations::permute(&permutation, &mut items);

    usize_to_u32(items)
}
