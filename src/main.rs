mod catalog;
mod k8s;
mod net;
mod protos;

#[macro_use]
extern crate log;

use catalog::Catalog;
use kube::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let mut server = net::server::NettingServer::new(String::from("0.0.0.0"), 50_051);
    server.start();
    for (host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    loop {}*/
    let client = Client::try_default().await?;
    let mut c = Catalog::new().await;
    c.build_catalog(client.clone()).await;
    let deployments = c.deploy_catalog.get("default").unwrap();
    let container = k8s::types::NettingContainer {
        name: "netting-tmp".to_owned(),
        image: "netting:v2".to_owned(),
        ports: vec![50_052],
    };
    for deploy in deployments {
        k8s::inject::inject_container_into_deploy(
            client.clone(),
            deploy.clone(),
            container.clone(),
        )
        .await;
    }
    //println!("{:?}", c);
    Ok(())
}
