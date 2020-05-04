use k8s_openapi::api::core::v1::Pod;

use kube::{
    api::{Api, ListParams, Meta},
    Client,
};

struct PodDetails {
    name: String,
}

pub async fn get_pods(client: Client, namespace: String) -> Result<(), kube::Error> {
    let pods: Api<Pod> = Api::namespaced(client, namespace.as_ref());
    let lp = ListParams::default().labels("");
    for p in pods.list(&lp).await? {
        println!("Pod: {:?}", Meta::meta(&p));
    }
    Ok(())
}
