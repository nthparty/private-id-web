extern crate private_id_common as common;
extern crate private_id_crypto as crypto;
extern crate private_id_protocol as protocol;
extern crate private_id_wrappers as wrappers;

// extern crate rand_core;
//
// use rand_core::OsRng;
// use rand_core::RngCore;

use crypto::prelude::TPayload;
use protocol::private_id::{partner::PartnerPrivateId, traits::*};
use protocol::private_id::{company::CompanyPrivateId, traits::CompanyPrivateIdProtocol};
use crypto::spoint::ByteBuffer;
use std::any::Any;

use wasm_bindgen::prelude::*;
use super::js::*;
use serde_json::json;

#[wasm_bindgen]
pub fn test() -> String {
    // let &mut input_with_headers;
    // let var_name = protocol::private_id::partner::PartnerPrivateId::load_data("", input_with_headers);
    // protocol::private_id::partner::PartnerPrivateId::new();
    let hello = "Hello".to_string();

    // format!("{}", json!(
    //     vec!([
    //         vec!([hello.clone(), hello.clone(), hello.clone()]),
    //         vec!([hello.clone(), hello.clone(), hello.clone()])
    //     ])
    // ))

    match wrappers::wrappers::main().unwrap() {
        () => "unit",
        _ => "error"
    }.to_string()

    // wrappers::wrappers::client_stage().to_string();



    // protocol::private_id::partner::PartnerPrivateId::new();

    // protocol::fileio::load_data()

    // "".to_string()
}

#[wasm_bindgen]
pub fn permute(p: Vec<u32>, a: Vec<u32>) -> Vec<u32> {
    let mut items: Vec<usize> = u32_to_usize(a);
    let permutation = u32_to_usize(p);

    common::permutations::permute(&permutation, &mut items);

    usize_to_u32(items)
}

#[wasm_bindgen]
pub fn run() -> String {

    let not_matched_val: Option<&str> = Option::Some("Unknown");
    let use_row_numbers = true;

    let partner_input = r#"[
        "sanderswilliam@watkins.org", "kim97@hotmail.com", "danielhernandez@hotmail.com",
        "bryanttanner@hotmail.com", "xmeza@white-ramsey.com", "marshallaustin@hotmail.com",
        "robinfreeman@yahoo.com", "portermark@yahoo.com", "david97@gmail.com",
        "showard@williamson-payne.net", "mclaughlintina@reynolds.com", "paul61@gmail.com",
        "walshkenneth@richard.org", "tyler77@yahoo.com", "willisalison@clark-williams.com",
        "joanna88@gmail.com", "rhernandez@thompson.com", "allentonya@barr.com",
        "miguel23@taylor-gilbert.com", "jacobparsons@reilly-ward.com", "bankscynthia@gmail.com",
        "rebeccajenkins@gmail.com", "nancyfields@irwin-sanders.com", "woodcourtney@hotmail.com",
        "xcombs@yahoo.com", "erik44@gmail.com"
    ]"#;

    let company_input = r#"[
        "rebeccajenkins@gmail.com", "mooneyamanda@hotmail.com", "bryanttanner@hotmail.com",
        "xcombs@yahoo.com", "brenda85@hotmail.com", "kim97@hotmail.com", "william23@hotmail.com",
        "aaron59@jones.net", "walshkenneth@richard.org", "woodcourtney@hotmail.com",
        "moliver@rush.com", "sanderswilliam@watkins.org", "bankscynthia@gmail.com",
        "robinfreeman@yahoo.com", "zlawrence@hotmail.com", "rhernandez@thompson.com",
        "willisalison@clark-williams.com", "jacobparsons@reilly-ward.com", "erik44@gmail.com",
        "edwardsgeorge@gmail.com", "catherinedavis@hotmail.com", "stephanie23@gmail.com",
        "tyler77@yahoo.com", "nancyfields@irwin-sanders.com", "portermark@yahoo.com",
        "raymond60@hotmail.com", "sandra41@moody.com", "joanna88@gmail.com",
        "greenstephanie@yahoo.com", "showard@williamson-payne.net"
    ]"#;

    PartnerPrivateId::new();

    company_input.to_string()
}

// #[wasm_bindgen]
// pub fn random() -> u32 {
//     let mut scalar_bytes = [0u8; 64];
//     OsRng.fill_bytes(&mut scalar_bytes);
//     1
//
//
//     // let mut os_rng = OsRng::new().unwrap();
//     // let mut key = [0u8; 16];
//     // os_rng.fill_bytes(&mut key);
//     // let random_u64 = os_rng.next_u32();
//     //
//     // random_u64
// }
