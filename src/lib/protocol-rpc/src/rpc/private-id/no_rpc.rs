extern crate log;

extern crate common;
extern crate crypto;
extern crate ctrlc;
extern crate protocol;
extern crate retry;
extern crate rpc;
extern crate tokio_rustls;
extern crate tonic;

use log::info;

use common::timer;
use crypto::prelude::TPayload;
use protocol::private_id::{partner::PartnerPrivateId, traits::*};

use protocol::private_id::{company::CompanyPrivateId, traits::CompanyPrivateIdProtocol};
use crypto::spoint::ByteBuffer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let global_timer = timer::Timer::new_silent("global");

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
    let partner_protocol = PartnerPrivateId::new();

    // 2. Load partner's data
    // 3. Generate permutation pattern
    // 4. Permute data and hash
    partner_protocol.load_data(partner_input, false).unwrap();
    partner_protocol.gen_permute_pattern().unwrap();
    let u_partner = partner_protocol.permute_hash_to_bytes().unwrap();

    // 5. Initialize company - this loads company's data and generates its permutation pattern
    let company_protocol = CompanyPrivateId::new();
    company_protocol.load_data(company_input, false);
    // company_protocol.gen_permute_pattern().unwrap();

    // 6. Get data from company
    let mut u_company: Vec<ByteBuffer> = TPayload::new();
    // rpc_client::recv().await.unwrap();  // tag name: "u_company".to_string()

    let _ = timer::Builder::new()
        .label("server")
        .extra_label("recv_u_company")
        .build();
    let t = timer::Builder::new().label("u_company").build();
    let res = company_protocol.get_permuted_keys().unwrap();
    t.qps(format!("received {}", "u_company").as_str(), res.len());

    u_company = /*receive(*/(res)/*)*/;

    // 7. Permute and encrypt data from company with own keys
    let (e_company, v_company) = partner_protocol.encrypt_permute(u_company);

    // 8. Send partner's data to company
    // let ack_u_partner = rpc_client::send(u_partner);  // tag name: "u_partner".to_string()
    let u_partner = /*receive(*/(u_partner)/*)*/;
    company_protocol.set_encrypted_partner_keys(u_partner).unwrap();

    // 9a. Send company's data back to company
    // let ack_e_company = rpc_client::send(e_company);  // tag name: "e_company".to_string()
    let e_company = /*receive(*/(e_company)/*)*/;
    company_protocol.set_encrypted_company("e_company".to_string(), e_company).unwrap();

    // 9b. Send company's data back to company
    // let ack_v_company = rpc_client::send(v_company);  // tag name: "v_company".to_string()
    let v_company = /*receive(*/(v_company)/*)*/;
    company_protocol.set_encrypted_company("v_company".to_string(), v_company).unwrap();

    // let step1_barrier = Step1Barrier {
    //     u_partner_ack: Some(ack_u_partner),
    //     e_company_ack: Some(ack_e_company),
    //     v_company_ack: Some(ack_v_company),
    // };

    // 10. Receive partner's back from company
    let mut v_partner = TPayload::new();
    // rpc_client::recv(&mut v_partner);  // "v_partner".to_string()
    v_partner = /*receive(*/(company_protocol.get_encrypted_partner_keys())/*)*/.unwrap();

    // 11. Calculate symmetric set difference between company and partners data
    // let calculate_set_diff_ack = rpc_client::calculate_set_diff();
    company_protocol.calculate_set_diff().unwrap();

    // 12. Get data that partner has but company doesn't
    let mut s_prime_partner = TPayload::new();
    // rpc_client::recv(&mut s_prime_partner);  // tag name: "s_prime_partner".to_string()
    s_prime_partner = /*
        receive(*/(company_protocol.get_set_diff_output("s_prime_partner".to_string()))/*)
                */.unwrap();


    // 13. Get data that company has but partner doesn't
    let mut s_prime_company = TPayload::new();
    // rpc_client::recv(&mut s_prime_company);  // tag name: "s_prime_company".to_string()
    s_prime_company = /*
        receive(*/(company_protocol.get_set_diff_output("s_prime_company".to_string()))/*)
                */.unwrap();

    // 14. Encrypt and send back data that partner has company doesn't.  Generates s_double_prime_partner in-place
    let mut s_prime_partner= partner_protocol.encrypt(s_prime_partner).unwrap();
    // rpc_client::send(partner_protocol.encrypt(s_prime_partner).unwrap());  // tag name: "s_double_prime_partner".to_string()
    s_prime_partner = /*receive(*/(s_prime_partner)/*)*/;
    company_protocol.write_partner_to_id_map(s_prime_partner, not_matched_val).unwrap();

    // 15. Create partner's ID spine and print
    partner_protocol.create_id_map(v_partner, s_prime_company, not_matched_val);
    partner_protocol.print_id_map(usize::MAX, false, use_row_numbers);

    // 16. Create company's ID spine and print
    // rpc_client::reveal();  // tag name: "reveal"
    global_timer.qps("total time", partner_protocol.get_size());

    // Print company's output
    company_protocol.write_company_to_id_map();
    company_protocol.print_id_map(u32::MAX as usize, false, use_row_numbers);

    info!("Bye!");
    Ok(())
}
