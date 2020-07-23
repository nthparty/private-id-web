
// MIT License

pub mod common {
    tonic::include_proto!("common");
}

pub mod gen_private_id {
    tonic::include_proto!("privateid");
}

pub mod streaming;

use gen_private_id::private_id_client::PrivateIdClient;
use tonic::transport::Channel;
pub enum RpcClient {
    PrivateId(PrivateIdClient<Channel>),
}

use crypto::{prelude::*};

pub mod from {
    use super::{common::*, *};

    impl From<&TPayload> for common::Payload {
        fn from(payload: &TPayload) -> Self {
            let z = payload
                .iter()
                .map(|c| c.buffer.clone())
                .collect::<Vec<Vec<u8>>>();
            common::Payload { payload: z }
        }
    }

    impl From<&Payload> for TPayload {
        fn from(pld: &Payload) -> Self {
            pld.payload
                .iter()
                .map(|x| ByteBuffer { buffer: x.to_vec() })
                .collect::<TPayload>()
        }
    }
}