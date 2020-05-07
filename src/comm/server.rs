use crate::protos::msg::*;
use crate::protos::msg_grpc::*;

use futures::future::Future;
use grpcio::{RpcContext, UnarySink};

#[derive(Clone)]
pub struct TalkerServer;

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
