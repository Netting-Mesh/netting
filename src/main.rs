mod k8s;
mod net;
mod protos;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = net::server::TalkerServer::new(String::from("127.0.0.1"), 50_051);
    server.start();
    for (host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    loop {}
    Ok(())
}
