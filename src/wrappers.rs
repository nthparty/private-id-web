extern crate private_id_common as common;
extern crate private_id_crypto as crypto;
extern crate private_id_protocol as protocol;
extern crate private_id_wrappers as rust_wrappers;

use wasm_bindgen::prelude::*;
use super::js::*;

use crypto::prelude::Bytes;
use self::rust_wrappers::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind console_log to `console.log(..)`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

pub fn protocol(
    partner_input: &str,
    company_input: &str,
    not_matched_val: Option<&str>,
    use_row_numbers: bool
) -> String {
    // 1. Create partner protocol instance
    // partner_init();

    // 2. Load partner's data
    // 3. Generate permutation pattern
    // 4. Permute data and hash
    partner_step_2(partner_input);
    partner_step_3();
    let u_partner: Bytes = partner_step_4();

    // 5. Initialize company - this loads company's data and generates its permutation pattern
    // company_init();
    company_step_5(company_input);

    // 6. Get data from company
    let mut u_company: Bytes = Bytes::new();
    // rpc_client::recv().await.unwrap();  // tag name: "u_company".to_string()

    let res: Bytes = company_step_6();

    // let res_json = res.to_json();
    // console_log(&res_json);
    // let res2 = Bytes::from_json(res_json);

    u_company = /*receive(*/(res)/*)*/;

    // 7. Permute and encrypt data from company with own keys
    let (e_company, v_company): (Bytes, Bytes) = partner_step_7(u_company);

    // 8. Send partner's data to company
    // let ack_u_partner = rpc_client::send(u_partner);  // tag name: "u_partner".to_string()
    let u_partner = /*receive(*/(u_partner)/*)*/;
    company_step_8(u_partner);

    // 9a. Send company's data back to company
    // let ack_e_company = rpc_client::send(e_company);  // tag name: "e_company".to_string()
    let e_company = /*receive(*/(e_company)/*)*/;
    company_step_9a(e_company);

    // 9b. Send company's data back to company
    // let ack_v_company = rpc_client::send(v_company);  // tag name: "v_company".to_string()
    let v_company = /*receive(*/(v_company)/*)*/;
    company_step_9b(v_company);

    // 10. Receive partner's back from company
    let mut v_partner = Bytes::new();
    // rpc_client::recv(&mut v_partner);  // "v_partner".to_string()
    v_partner = /*receive(*/(company_step_10())/*)*/;

    // 11. Calculate symmetric set difference between company and partners data
    // let calculate_set_diff_ack = rpc_client::calculate_set_diff();
    company_step_11();

    // 12. Get data that partner has but company doesn't
    let mut s_prime_partner = Bytes::new();
    // rpc_client::recv(&mut s_prime_partner);  // tag name: "s_prime_partner".to_string()
    s_prime_partner = /*    receive(*/(company_step_12())/*)    */;

    // 13. Get data that company has but partner doesn't
    let mut s_prime_company = Bytes::new();
    // rpc_client::recv(&mut s_prime_company);  // tag name: "s_prime_company".to_string()
    s_prime_company = /*    receive(*/(company_step_13())/*)    */;

    // 14. Encrypt and send back data that partner has and company doesn't.  Generates s_double_prime_partner in-place
    let mut s_prime_partner = partner_step_14(s_prime_partner);
    // rpc_client::send(partner_step_14(s_prime_partner));  // tag name: "s_double_prime_partner".to_string()
    s_prime_partner = /*    receive(*/(s_prime_partner)/*)    */;
    company_step_14(s_prime_partner, not_matched_val);

    // 15. Create partner's ID spine and print
    partner_step_15(v_partner, s_prime_company, not_matched_val);

    company_step_15();

    // 16. Create company's ID spine and print
    // rpc_client::reveal();  // tag name: "reveal"

    let mut output = "".to_owned();
    output.push_str(&company_build_output(use_row_numbers));
    output.push_str("\n");
    output.push_str(&partner_build_output(use_row_numbers));
    output
}

#[wasm_bindgen]
pub fn test(n: u32) -> String {
    let not_matched_val: Option<&str> = Option::Some("Unknown");
    let use_row_numbers = true;

    let mut data: String = "".to_owned();
    for i in 1..n {
        data.push_str(&format!("\"shared{}email@example.org\",", i));
    }

    let partner_input = &format!("[{}\"partnersonly@gmail.com\"]", data.clone());
    let company_input = &format!("[{}\"sealevel@company.net\"]", data.clone());

    protocol(partner_input, company_input, not_matched_val, use_row_numbers)
}

#[wasm_bindgen]
pub fn permute(p: Vec<u32>, a: Vec<u32>) -> Vec<u32> {
    let mut items: Vec<usize> = u32_to_usize(a);
    let permutation = u32_to_usize(p);

    common::permutations::permute(&permutation, &mut items);

    usize_to_u32(items)
}

#[wasm_bindgen]
pub fn run(partner_input: String, company_input: String, not_matched_val: String, use_row_numbers: bool)
    -> String {
    let not_matched_val = Option::Some(&not_matched_val as &str);
    protocol(
        &partner_input,
        &company_input,
        not_matched_val,
        use_row_numbers
    )
}
