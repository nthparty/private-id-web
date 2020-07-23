
// MIT License

extern crate clap;
extern crate common;
extern crate crypto;
extern crate ctrlc;
extern crate protocol;
extern crate retry;
extern crate rpc;
extern crate tokio_rustls;
extern crate tonic;

use log::info;
use tonic::Request;

use common::timer;
use crypto::prelude::TPayload;
use protocol::private_id::{partner::PartnerPrivateId, traits::*};
use rpc::{
    connect::create_client::create_client,
    proto::{
        gen_private_id::{service_response::*, Init, ServiceResponse, Step1Barrier},
        RpcClient,
    },
};

mod rpc_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let global_timer = timer::Timer::new_silent("global");
    let input_path = "example/email_partner.csv";
    let input_with_headers = false;
    let output_path: Option<&str> = None;
    let na_val: Option<&str> = None;
    let use_row_numbers = true;

    let mut client_context = {
        let no_tls = true;
        let host_pre: Option<&str> = Option::Some("localhost:3001");
        let tls_dir: Option<&str> = None;
        let tls_key: Option<&str> = None;
        let tls_cert: Option<&str> = None;
        let tls_ca: Option<&str> = None;
        let tls_domain: Option<&str> = None;

        let RpcClient::PrivateId(x) = create_client(
            no_tls,
            host_pre,
            tls_dir,
            tls_key,
            tls_cert,
            tls_ca,
            tls_domain,
            "private-id".to_string(),
        );
        x
    };

    info!("Input path: {}", input_path);
    if output_path.is_some() {
        info!("Output path: {}", output_path.unwrap());
    } else {
        info!("Output view to stdout (first 10 items)");
    }

    // 1. Create partner protocol instance
    let partner_protocol = PartnerPrivateId::new();

    // 2. Load partner's data
    // 3. Generate permute pattern
    // 4. Permute data and hash
    partner_protocol
        .load_data(&input_path.to_string(), input_with_headers)
        .unwrap();
    partner_protocol.gen_permute_pattern().unwrap();
    let u_partner = partner_protocol.permute_hash_to_bytes().unwrap();

    // 5. Initialize company - this loads company's data and generates its permutation pattern
    let init_ack = match client_context
        .initialize(Request::new(Init {}))
        .await?
        .into_inner()
        .ack
        .unwrap()
    {
        Ack::InitAck(x) => x,
        _ => panic!("wrong ack"),
    };

    // 6. Get data from company
    let mut u_company = TPayload::new();
    let _ = rpc_client::recv(
        ServiceResponse {
            ack: Some(Ack::InitAck(init_ack.clone())),
        },
        "u_company".to_string(),
        &mut u_company,
        &mut client_context,
    )
    .await?;

    // 7. Permute and encrypt data from company with own keys
    let (e_company, v_company) = partner_protocol.encrypt_permute(u_company);

    // 8. Send partner's data to company
    let ack_u_partner =
        match rpc_client::send(u_partner, "u_partner".to_string(), &mut client_context)
            .await?
            .into_inner()
            .ack
            .unwrap()
        {
            Ack::UPartnerAck(x) => x,
            _ => panic!("wrong ack"),
        };

    // 9a. Send company's data back to company
    let ack_e_company =
        match rpc_client::send(e_company, "e_company".to_string(), &mut client_context)
            .await?
            .into_inner()
            .ack
            .unwrap()
        {
            Ack::ECompanyAck(x) => x,
            _ => panic!("wrong ack"),
        };

    // 9b. Send company's data back to company
    let ack_v_company =
        match rpc_client::send(v_company, "v_company".to_string(), &mut client_context)
            .await?
            .into_inner()
            .ack
            .unwrap()
        {
            Ack::VCompanyAck(x) => x,
            _ => panic!("wrong ack"),
        };

    let step1_barrier = Step1Barrier {
        u_partner_ack: Some(ack_u_partner),
        e_company_ack: Some(ack_e_company),
        v_company_ack: Some(ack_v_company),
    };

    // 10. Receive partner's back from company
    let mut v_partner = TPayload::new();
    let _ = rpc_client::recv(
        ServiceResponse {
            ack: Some(Ack::Step1Barrier(step1_barrier.clone())),
        },
        "v_partner".to_string(),
        &mut v_partner,
        &mut client_context,
    )
    .await?;

    // 11. Calculate symmetric set difference between company and partners data
    let calculate_set_diff_ack =
        match rpc_client::calculate_set_diff(step1_barrier.clone(), &mut client_context)
            .await?
            .into_inner()
            .ack
            .unwrap()
        {
            Ack::CalculateSetDiffAck(x) => x,
            _ => panic!("wrong ack"),
        };

    // 12. Get data that partner has but company doesn't
    let mut s_prime_partner = TPayload::new();
    let _ = rpc_client::recv(
        ServiceResponse {
            ack: Some(Ack::CalculateSetDiffAck(calculate_set_diff_ack.clone())),
        },
        "s_prime_partner".to_string(),
        &mut s_prime_partner,
        &mut client_context,
    )
    .await?;

    // 13. Get data that company has but partner doesn't
    let mut s_prime_company = TPayload::new();
    let _ = rpc_client::recv(
        ServiceResponse {
            ack: Some(Ack::CalculateSetDiffAck(calculate_set_diff_ack.clone())),
        },
        "s_prime_company".to_string(),
        &mut s_prime_company,
        &mut client_context,
    )
    .await?;

    // 14. Encrypt and send back data that partner has company doesn't
    //     Generates s_double_prime_partner in-place
    let _ = rpc_client::send(
        partner_protocol.encrypt(s_prime_partner)?,
        "s_double_prime_partner".to_string(),
        &mut client_context,
    )
    .await?
    .into_inner()
    .ack
    .unwrap();

    // 15. Create partner's ID spine and print
    partner_protocol.create_id_map(v_partner, s_prime_company, na_val);
    match output_path {
        Some(p) => partner_protocol
            .save_id_map(&String::from(p), input_with_headers, use_row_numbers)
            .unwrap(),
        None => partner_protocol.print_id_map(usize::MAX, input_with_headers, use_row_numbers),
    }

    // 16. Create company's ID spine and print
    rpc_client::reveal(&mut client_context).await?;
    global_timer.qps("total time", partner_protocol.get_size());
    info!("Bye!");
    Ok(())
}
