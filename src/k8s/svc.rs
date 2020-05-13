use crate::k8s::pod::{get_pod_details, get_pod_list};
use crate::k8s::types::*;
use k8s_openapi::api::core::v1::Service;
use kube::{
    api::{Api, ListParams},
    Client,
};

pub async fn get_svc_list(client: Client, namespace: String) -> Result<Vec<Service>, kube::Error> {
    let svcs: Api<Service> = Api::namespaced(client, namespace.as_ref());
    let lp = ListParams::default().labels("");
    let mut ret = Vec::new();
    for s in svcs.list(&lp).await? {
        ret.push(s);
    }
    Ok(ret)
}

pub async fn get_svc_details(svc: Service, client: Client) -> Result<NettingService, &'static str> {
    let mut labels = String::new();
    match svc.spec.clone().unwrap().selector {
        Some(_) => {
            for (key, value) in svc.spec.clone().unwrap().selector.unwrap() {
                labels.push_str(format!("{}={},", key, value).as_ref());
            }
            labels.pop();
        }
        None => return Err("No labels found in service, doesn't expose anything"),
    }
    let pods = get_pod_list(
        client,
        svc.metadata.clone().unwrap().namespace.unwrap(),
        labels,
    )
    .await;
    let mut netting_pods = Vec::new();
    for pod in pods.unwrap() {
        netting_pods.push(get_pod_details(pod, true).await);
    }
    Ok(NettingService {
        name: svc.metadata.clone().unwrap().name.unwrap(),
        namespace: svc.metadata.clone().unwrap().namespace.unwrap(),
        cluster_ip: svc.spec.unwrap().cluster_ip.unwrap(),
        pods_exposed: netting_pods,
    })
}
