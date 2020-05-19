use crate::k8s::pod::{get_pod_details, get_pod_list};
use crate::k8s::types::*;
use k8s_openapi::api::core::v1::Service;
use kube::{
    api::{Api, ListParams},
    Client,
};

/// Returns a list of Service objects within
/// the given namespace filtered by the labels
pub async fn get_svc_list(
    client: Client,
    namespace: String,
    labels: String,
) -> Result<Vec<Service>, kube::Error> {
    let svcs: Api<Service> = Api::namespaced(client, namespace.as_ref());
    let lp = ListParams::default().labels(labels.as_ref());
    let mut ret = Vec::new();
    for s in svcs.list(&lp).await? {
        ret.push(s);
    }
    Ok(ret)
}

/// Returns a wrapper object for the given
/// Service object
pub async fn get_svc_details(svc: Service, client: Client) -> Result<NettingService, kube::Error> {
    if let Some(spec) = svc.spec {
        let mut labels = String::new();
        if let Some(selector) = spec.selector {
            for (key, value) in selector {
                labels.push_str(format!("{}={},", key, value).as_ref());
            }
            labels.pop(); // Remove trailing comma
        } else {
            return Err(kube::Error::Kubeconfig(
                "No labels found in service, doesn't expose anything".to_owned(),
            ));
        }
        if let Some(metadata) = svc.metadata {
            let pods = get_pod_list(
                client,
                metadata
                    .namespace
                    .clone()
                    .expect("Namespace undefined in Service"),
                labels,
            )
            .await?;
            let mut netting_pods = Vec::new();
            for pod in pods {
                netting_pods.push(get_pod_details(pod, true).await?);
            }
            Ok(NettingService {
                name: metadata.name.expect("Name undefined in Service"),
                namespace: metadata.namespace.expect("Namespace undefined in Service"),
                cluster_ip: spec.cluster_ip.expect("Cluster_ip undefined in Service"),
                pods_exposed: netting_pods,
            })
        } else {
            return Err(kube::Error::Kubeconfig(
                "Service metadata undefined".to_owned(),
            ));
        }
    } else {
        return Err(kube::Error::Kubeconfig("Service spec undefined".to_owned()));
    }
}
