mod comm;
mod k8;
mod protos;

#[macro_use]
extern crate log;

use comm::server::TalkerServer;
use grpcio::{ChannelBuilder, Environment, ServerBuilder};
use protos::msg_grpc::create_talker;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Arc::new(Environment::new(1));
    let talker = create_talker(TalkerServer);
    let ch_builder = ChannelBuilder::new(env.clone());
    let mut server = ServerBuilder::new(env)
        .register_service(talker)
        .bind("127.0.0.1", 50_051)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();
    for (host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    loop {}
    Ok(())
}
