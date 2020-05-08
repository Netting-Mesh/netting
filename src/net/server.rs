use crate::protos::msg::*;
use crate::protos::msg_grpc::*;

use futures::future::Future;
use grpcio::{ChannelBuilder, Environment, RpcContext, Server, ServerBuilder, UnarySink};
use std::sync::Arc;

#[derive(Clone)]
pub struct TalkerServer {
    status: i32,
}

impl Talker for TalkerServer {
    fn talk(&mut self, ctx: RpcContext<'_>, req: InitSystem, sink: UnarySink<InitSystem>) {
        let mut resp = InitSystem::new();
        println!("Recevied message from {}", req.get_ip_address());
        resp.set_ip_address(String::from("Netting"));
        let f = sink
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

impl TalkerServer {
    pub fn new(ip: String, port: u16) -> Server {
        let env = Arc::new(Environment::new(1));
        let talker = create_talker(TalkerServer { status: 1 });
        let ch_builder = ChannelBuilder::new(env.clone());
        let mut server = ServerBuilder::new(env)
            .register_service(talker)
            .bind(ip, port)
            .channel_args(ch_builder.build_args())
            .build()
            .unwrap();
        server
    }
}
