mod k8;
use k8::pod::get_pods;
use k8::svc::get_svcs;
use kube::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let namespace = std::env::var("NAMESPACE").unwrap_or("kube-system".into());
    //get_pods(client, namespace).await?;
    get_svcs(client, namespace).await?;
    Ok(())
}
