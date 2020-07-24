#[macro_use]
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

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};

mod rpc_client;
mod rpc_server;
use rpc::{connect::create_server::create_server, proto::gen_private_id::private_id_server};

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

    let host = "0.0.0.0:3001";
    let host_pre: &str = "localhost:3001";

    let RpcClient::PrivateId(mut client_context) = create_client(
        true,
        Option::Some(host_pre),
        None,
        None,
        None,
        None,
        None,
        "private-id".to_string(),
    );
    let (mut server, tx, rx) = create_server(
        true,
        None,
        None,
        None,
        None
    );


    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
        .expect("Error setting Ctrl-C handler");

    let service = rpc_server::PrivateIdService::new(
        company_input,
        None,
        false,
        not_matched_val,
        use_row_numbers,
    );

    let ks = service.killswitch.clone();
    let recv_thread = thread::spawn(move || {
        let sleep_dur = time::Duration::from_millis(1000);
        while !(ks.load(Ordering::Relaxed)) && running.load(Ordering::Relaxed) {
            thread::sleep(sleep_dur);
        }

        info!("Shutting down server ...");
        tx.send(()).unwrap();
    });

    info!("Server starting at {}", host);

    let addr = host.parse().unwrap();

    server
        .add_service(private_id_server::PrivateIdServer::new(service))
        .serve_with_shutdown(addr, async {
            rx.await.ok();
        })
        .await.unwrap();

    recv_thread.join().unwrap();
    info!("Bye!");
    println!("end server_thread");





    // 1. Create partner protocol instance
    let partner_protocol = PartnerPrivateId::new();

    // 2. Load partner's data
    // 3. Generate permutation pattern
    // 4. Permute data and hash
    partner_protocol
        .load_data(partner_input, false)
        .unwrap();
    partner_protocol.gen_permute_pattern().unwrap();
    let u_partner = partner_protocol.permute_hash_to_bytes().unwrap();

    // 5. Initialize company - this loads company's data and generates its permutation pattern
    let init_ack = match client_context
        .initialize(Request::new(Init {}))
        .await.unwrap()
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
        .await.unwrap();

    // 7. Permute and encrypt data from company with own keys
    let (e_company, v_company) = partner_protocol.encrypt_permute(u_company);

    // 8. Send partner's data to company
    let ack_u_partner =
        match rpc_client::send(u_partner, "u_partner".to_string(), &mut client_context)
            .await.unwrap()
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
            .await.unwrap()
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
            .await.unwrap()
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
        .await.unwrap();

    // 11. Calculate symmetric set difference between company and partners data
    let calculate_set_diff_ack =
        match rpc_client::calculate_set_diff(step1_barrier.clone(), &mut client_context)
            .await.unwrap()
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
        .await.unwrap();

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
        .await.unwrap();

    // 14. Encrypt and send back data that partner has company doesn't
    //     Generates s_double_prime_partner in-place
    let _ = rpc_client::send(
        partner_protocol.encrypt(s_prime_partner).unwrap(),
        "s_double_prime_partner".to_string(),
        &mut client_context,
    )
        .await.unwrap()
        .into_inner()
        .ack
        .unwrap();

    // 15. Create partner's ID spine and print
    partner_protocol.create_id_map(v_partner, s_prime_company, not_matched_val);
    partner_protocol.print_id_map(usize::MAX, false, use_row_numbers);

    // 16. Create company's ID spine and print
    rpc_client::reveal(&mut client_context).await.unwrap();
    global_timer.qps("total time", partner_protocol.get_size());
    info!("Bye!");
    println!("end client_thread");

    Ok(())
}
