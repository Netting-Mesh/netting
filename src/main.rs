mod k8;
mod protos;

#[macro_use]
extern crate log;

use k8::pod::get_pods;
use k8::svc::get_svcs;
use kube::Client;

use protos::msg::*;
use protos::msg_grpc::*;

use futures::future::Future;

use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let namespace = std::env::var("NAMESPACE").unwrap_or("kube-system".into());
    //get_pods(client, namespace).await?;
    get_svcs(client, namespace).await?;
    loop {}
    Ok(())
}

struct TalkerServer;
/*
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
}*/
