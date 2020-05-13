use k8s_openapi::api::core::v1::Namespace;
use kube::{
    api::{Api, ListParams},
    Client,
};

pub async fn get_namespaces(client: Client) -> Result<Vec<String>, kube::Error> {
    let namespaces: Api<Namespace> = Api::all(client);
    let lp = ListParams::default().labels("");
    let mut ret = Vec::new();
    for n in namespaces.list(&lp).await? {
        ret.push(n.metadata.unwrap().name.unwrap());
    }
    Ok(ret)
}
