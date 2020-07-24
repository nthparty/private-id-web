// MIT License

#[macro_use]
extern crate log;
extern crate clap;
extern crate ctrlc;
extern crate tonic;

use log::info;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};

mod rpc_server;
use rpc::{connect::create_server::create_server, proto::gen_private_id::private_id_server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let not_matched_val: Option<&str> = Option::Some("Unknown");
    let use_row_numbers = true;
    let input = r#"[
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
        input,
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

    let addr = host.parse()?;

    server
        .add_service(private_id_server::PrivateIdServer::new(service))
        .serve_with_shutdown(addr, async {
            rx.await.ok();
        })
        .await?;

    recv_thread.join().unwrap();
    info!("Bye!");
    Ok(())
}
