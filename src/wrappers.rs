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

    #[wasm_bindgen(js_namespace = __rust__, js_name = progress_setPercent)]
    fn progress(p: u32);

    #[wasm_bindgen(js_namespace = __rust__, js_name = progress_message)]
    fn info(s: &str);
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

fn partner_phase_1(partner_input: &str) {
    // 1. Create partner protocol instance
    // 2. Load partner's data
    // 3. Generate permutation pattern
    partner_step_2(partner_input);
    partner_step_3();
}

fn company_phase_1(company_input: &str) -> Bytes {
    // 5. Initialize company - this loads company's data and generates its permutation pattern
    // company_init();
    company_step_5(company_input);

    // 6. Get data from company
    let u_company: Bytes = company_step_6();

    u_company
}

fn partner_phase_2(u_company: Bytes) -> (Bytes, Bytes, Bytes) {
    // 4. Permute data and hash
    let u_partner: Bytes = partner_step_4();

    // 6. Get data from company

    // 7. Permute and encrypt data from company with own keys
    let (e_company, v_company): (Bytes, Bytes) = partner_step_7(u_company);

    (u_partner, e_company, v_company)
}

fn company_phase_2(u_partner: Bytes, e_company: Bytes, v_company: Bytes) -> (Bytes, Bytes, Bytes) {
    // 8. Send partner's data to company
    company_step_8(u_partner);

    // 9a. Send company's data back to company
    company_step_9a(e_company);

    // 9b. Send company's data back to company
    company_step_9b(v_company);

    // 10. Receive partner's back from company
    let v_partner: Bytes = company_step_10();

    // 11. Calculate symmetric set difference between company and partners data
    company_step_11();

    // 12. Get data that partner has but company doesn't
    let s_prime_partner = company_step_12();

    // 13. Get data that company has but partner doesn't
    let s_prime_company: Bytes = company_step_13();

    (v_partner, s_prime_partner, s_prime_company)
}

fn partner_phase_3(s_prime_partner: Bytes) -> Bytes {
    // 14. Encrypt and send back data that partner has and company doesn't.  Generates s_double_prime_partner in-place
    let s_double_prime_partner = partner_step_14(s_prime_partner);

    s_double_prime_partner
}

fn company_phase_3(s_prime_partner: Bytes, not_matched_val: Option<&str>, use_row_numbers: bool) -> String {
    // 14. Encrypt and send back data that partner has and company doesn't.  Generates s_double_prime_partner in-place
    company_step_14(s_prime_partner, not_matched_val);

    // 15. Create company's ID spine and print
    company_step_15();

    company_build_output(use_row_numbers)
}

fn partner_phase_4(v_partner: Bytes, s_prime_company: Bytes, not_matched_val: Option<&str>, use_row_numbers: bool) -> String {
    // 15. Create partner's ID spine and print
    partner_step_15(v_partner, s_prime_company, not_matched_val);

    partner_build_output(use_row_numbers)
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
pub fn test_stages(n: u32) -> String {
    let not_matched_val: Option<&str> = Option::Some("Unknown");
    let use_row_numbers = true;

    let mut data: String = "".to_owned();
    for i in 1..n {
        data.push_str(&format!("\"shared{}email@example.org\",", i));
    }

    let partner_input = &format!("[{}\"partnersonly@gmail.com\"]", data.clone());
    let company_input = &format!("[{}\"sealevel@company.net\"]", data.clone());

    /* Phase 1 */
    partner_phase_1(partner_input);

    let u_company: Bytes = company_phase_1(company_input);
    let u_company_json = u_company.to_json_gz();

    /* Phase 2 */
    let u_company = Bytes::from_json_gz(u_company_json);
    let (u_partner, e_company, v_company) = partner_phase_2(u_company);
    let (u_partner_json, e_company_json, v_company_json) = (
        u_partner.to_json_gz(),
        e_company.to_json_gz(),
        v_company.to_json_gz()
    );

    let (u_partner, e_company, v_company) = (
        Bytes::from_json_gz(u_partner_json),
        Bytes::from_json_gz(e_company_json),
        Bytes::from_json_gz(v_company_json)
    );
    let (v_partner, s_prime_partner, s_prime_company) = company_phase_2(u_partner, e_company, v_company);
    let (v_partner_json, s_prime_partner_json, s_prime_company_json) = (
        v_partner.to_json_gz(),
        s_prime_partner.to_json_gz(),
        s_prime_company.to_json_gz()
    );

    /* Phase 3 */
    let s_prime_partner = Bytes::from_json_gz(s_prime_partner_json);
    let s_double_prime_partner = partner_phase_3(s_prime_partner);
    let s_double_prime_partner_json = s_double_prime_partner.to_json_gz();

    let s_double_prime_partner = Bytes::from_json_gz(s_double_prime_partner_json);
    let company_id_spine = company_phase_3(s_double_prime_partner, not_matched_val, use_row_numbers);

    /* Phase 4 */
    let (v_partner, s_prime_company) = (
        Bytes::from_json_gz(v_partner_json),
        Bytes::from_json_gz(s_prime_company_json)
    );
    let partner_id_spine = partner_phase_4(v_partner, s_prime_company, not_matched_val, use_row_numbers);

    partner_id_spine
}

#[wasm_bindgen]
pub fn partner_stage_1(partner_input: String) {
    console_log("1p");
    progress(10);
    info("Processing the partner's input...");
    partner_phase_1(&partner_input);
    progress(20);
    info("Input loaded.  Continuing...");
    console_log("1p");
}

#[wasm_bindgen]
pub fn company_stage_1(company_input: String) -> String {
    console_log("1c");
    info("Processing the company's input...");
    progress(10);
    let u_company: Bytes = company_phase_1(&company_input);
    let u_company_json = u_company.to_json_gz();
    console_log("1c");
    info("Input loaded.  Continuing...");
    progress(20);
    u_company_json
}

#[wasm_bindgen]
pub fn partner_stage_2(u_company_json: String) -> String {
    console_log("2p");
    progress(30);
    info("Permuting partner's data...");
    let u_company = Bytes::from_json_gz(u_company_json);

    info("Permuting and encrypting the company's data with the partner's own keys...");
    let (u_partner, e_company, v_company) = partner_phase_2(u_company);
    progress(40);

    let mut json = "".to_owned();
    json.push_str(&u_partner.to_json_gz());
    json.push_str("|");
    json.push_str(&e_company.to_json_gz());
    json.push_str("|");
    json.push_str(&v_company.to_json_gz());
    console_log("2p");
    info("Waiting for the company to compute our symmetric set difference...");
    json
}

#[wasm_bindgen]
pub fn company_stage_2(u_partner_json: String, e_company_json: String, v_company_json: String) -> String {
    console_log("2c");
    progress(30);
    info("Calculating symmetric set difference between the company's and partner's data..");
    let (u_partner, e_company, v_company) = (
        Bytes::from_json_gz(u_partner_json),
        Bytes::from_json_gz(e_company_json),
        Bytes::from_json_gz(v_company_json)
    );

    let (v_partner, s_prime_partner, s_prime_company) = company_phase_2(u_partner, e_company, v_company);
    info("Sent set difference to the partner.  Continuing...");
    progress(40);

    let mut json = "".to_owned();
    json.push_str(&v_partner.to_json_gz());
    json.push_str("|");
    json.push_str(&s_prime_partner.to_json_gz());
    json.push_str("|");
    json.push_str(&s_prime_company.to_json_gz());
    console_log("2c");
    json
}

#[wasm_bindgen]
pub fn partner_stage_3(s_prime_partner_json: String) -> String {
    console_log("3p");
    progress(60);
    info("Encrypting the data we have again.");
    let s_prime_partner = Bytes::from_json_gz(s_prime_partner_json);

    let s_double_prime_partner = partner_phase_3(s_prime_partner);

    let s_double_prime_partner_json = s_double_prime_partner.to_json_gz();
    console_log("3p");
    info("Waiting to receive the company's twice-encrypted data.");
    progress(75);

    s_double_prime_partner_json
}

#[wasm_bindgen]
pub fn company_stage_3(s_double_prime_partner_json: String, not_matched_val: String, use_row_numbers: bool) -> String {
    console_log("3c");
    progress(75);
    info("Received back data that partner has and company doesn't.  Continuing....  Continuing...");
    let s_double_prime_partner = Bytes::from_json_gz(s_double_prime_partner_json);

    let not_matched_val = Option::Some(&not_matched_val as &str);

    let company_id_spine = company_phase_3(s_double_prime_partner, not_matched_val, use_row_numbers);
    console_log("3c");
    info("Done.  Outputting the company's ID spine.  Printing...");
    progress(90);

    company_id_spine
}

#[wasm_bindgen]
pub fn partner_stage_4(v_partner_json: String, s_prime_company_json: String, not_matched_val: String, use_row_numbers: bool) -> String {
    console_log("4p");
    progress(80);
    info("Creating the ID spine from the company's data...");
    let (v_partner, s_prime_company) = (
        Bytes::from_json_gz(v_partner_json),
        Bytes::from_json_gz(s_prime_company_json)
    );

    let not_matched_val = Option::Some(&not_matched_val as &str);

    let partner_id_spine = partner_phase_4(v_partner, s_prime_company, not_matched_val, use_row_numbers);
    console_log("4p");
    info("Done.  Outputting the partner's ID spine.");
    progress(90);

    partner_id_spine
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
