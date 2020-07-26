#[macro_use]
extern crate lazy_static;

extern crate common;
extern crate crypto;
extern crate protocol;

use crypto::prelude::Bytes;
use protocol::private_id::{partner::PartnerPrivateId, traits::*};
use protocol::private_id::{company::CompanyPrivateId, traits::CompanyPrivateIdProtocol};
use crypto::spoint::ByteBuffer;

use std::sync::{Mutex, MutexGuard};

macro_rules! MUT { ($var:expr) => { $var.lock().unwrap() } }

lazy_static! {
    static ref partner: Mutex<PartnerPrivateId> = Mutex::new(PartnerPrivateId::new());
    static ref company: Mutex<CompanyPrivateId> = Mutex::new(CompanyPrivateId::new());
}

fn partner_step_2(partner_input: &str) {
    MUT!(partner).load_data(partner_input, false).unwrap();
}

fn partner_step_3() {
    MUT!(partner).gen_permute_pattern().unwrap();
}

fn partner_step_4() -> Bytes {
    MUT!(partner).permute_hash_to_bytes().unwrap()
}

fn company_step_5(company_input: &str) {
    MUT!(company).load_data(company_input, false);
    // MUT!(company).gen_permute_pattern().unwrap();
}

fn company_step_6() -> Bytes {
    MUT!(company).get_permuted_keys().unwrap()
}

fn partner_step_7(u_company: Bytes) -> (Bytes, Bytes) {
    MUT!(partner).encrypt_permute(u_company)
}

fn company_step_8(u_partner: Bytes) {
    MUT!(company).set_encrypted_partner_keys(u_partner).unwrap();
}

fn company_step_9a(e_company: Bytes) {
    MUT!(company).set_encrypted_company("e_company".to_string(), e_company).unwrap();
}

fn company_step_9b(v_company: Bytes) {
    MUT!(company).set_encrypted_company("v_company".to_string(), v_company).unwrap();
}

fn company_step_10() -> Bytes {
    MUT!(company).get_encrypted_partner_keys().unwrap()
}

fn company_step_11() {
    MUT!(company).calculate_set_diff().unwrap();
}

fn company_step_12() -> Bytes {
    MUT!(company).get_set_diff_output("s_prime_partner".to_string()).unwrap()
}

fn company_step_13() -> Bytes {
    MUT!(company).get_set_diff_output("s_prime_company".to_string()).unwrap()
}

fn company_step_14(s_prime_partner: Bytes, not_matched_val: Option<&str>) {
    MUT!(company).write_partner_to_id_map(s_prime_partner, not_matched_val).unwrap();
}

fn company_step_15() {
    MUT!(company).write_company_to_id_map();
}

fn company_print_output(use_row_numbers: bool) {
    MUT!(company).print_id_map(u32::MAX as usize, false, use_row_numbers);
}

fn partner_step_14(s_prime_partner: Bytes) -> Bytes {
    MUT!(partner).encrypt(s_prime_partner).unwrap()
}

fn partner_step_15(v_partner: Bytes, s_prime_company: Bytes, not_matched_val: Option<&str>) {
    MUT!(partner).create_id_map(v_partner, s_prime_company, not_matched_val);
}

fn partner_print_output(use_row_numbers: bool) {
    MUT!(partner).print_id_map(usize::MAX, false, use_row_numbers);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // let step1_barrier = Step1Barrier {
    //     u_partner_ack: Some(ack_u_partner),
    //     e_company_ack: Some(ack_e_company),
    //     v_company_ack: Some(ack_v_company),
    // };

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
    let mut s_prime_partner= partner_step_14(s_prime_partner);
    // rpc_client::send(partner_step_14(s_prime_partner));  // tag name: "s_double_prime_partner".to_string()
    s_prime_partner = /*    receive(*/(s_prime_partner)/*)    */;
    company_step_14(s_prime_partner, not_matched_val);

    // 15. Create partner's ID spine and print
    partner_step_15(v_partner, s_prime_company, not_matched_val);
    partner_print_output(use_row_numbers);

    // 16. Create company's ID spine and print
    // rpc_client::reveal();  // tag name: "reveal"

    // Print company's output
    company_step_15();
    company_print_output(use_row_numbers);

    Ok(())
}
