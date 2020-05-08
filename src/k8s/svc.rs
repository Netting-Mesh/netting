use k8s_openapi::api::core::v1::Service;
use kube::{
    api::{Api, ListParams, Meta},
    Client,
};

struct ServiceDetails {}

pub async fn get_svcs(client: Client, namespace: String) -> Result<(), kube::Error> {
    let svcs: Api<Service> = Api::namespaced(client, namespace.as_ref());
    let lp = ListParams::default().labels("");
    for s in svcs.list(&lp).await? {
        println!("Service: {:?}\n", Meta::meta(&s));
    }
    Ok(())
}
