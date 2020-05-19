use crate::k8s::types::*;
use k8s_openapi::api::core::v1::{Container, Pod};
use kube::{
    api::{Api, ListParams},
    Client,
};
use std::collections::HashSet;

/// Returns a wrapper object for the given
/// Container object
pub async fn get_container_details(container: Container) -> Result<NettingContainer, kube::Error> {
    let mut ports = Vec::new();
    if let Some(container_ports) = container.ports {
        for cp in container_ports {
            ports.push(cp.container_port);
        }
    }
    Ok(NettingContainer {
        name: container.name,
        image: container.image.expect("Image undefined in container"),
        ports: ports,
    })
}

/// Returns the container ready status
pub async fn get_container_status(pod: Pod) -> Result<String, kube::Error> {
    if let Some(status) = pod.status {
        if let Some(conditions) = status.conditions {
            for condition in conditions {
                if condition.type_ == "Ready" {
                    return Ok(condition.status);
                }
            }
        }
    }
    return Err(kube::Error::Kubeconfig(
        "Ready status undefined in container".to_owned(),
    ));
}

/// Returns a wrapper object for the given
/// Pod object
pub async fn get_pod_details(pod: Pod, exposed: bool) -> Result<NettingPod, kube::Error> {
    if let Some(spec) = pod.clone().spec {
        let mut containers = Vec::new();
        for container in spec.containers {
            containers.push(get_container_details(container).await?);
        }
        let status = get_container_status(pod.clone()).await?;
        if let Some(metadata) = pod.metadata {
            Ok(NettingPod {
                name: metadata.name.expect("Name undefined in Pod"),
                namespace: metadata.namespace.expect("Namespace undefined in Pod"),
                replicaset: "".to_owned(),
                containers: containers,
                status: status,
                exposed: exposed,
            })
        } else {
            return Err(kube::Error::Kubeconfig("Pod metadata undefined".to_owned()));
        }
    } else {
        return Err(kube::Error::Kubeconfig("Pod spec undefined".to_owned()));
    }
}

/// Returns a list of Pod objects within
/// the given namespace filtered by the labels
pub async fn get_pod_list(
    client: Client,
    namespace: String,
    labels: String,
) -> Result<Vec<Pod>, kube::Error> {
    let pods: Api<Pod> = Api::namespaced(client, namespace.as_ref());
    let lp = ListParams::default().labels(labels.as_ref());
    let mut ret = Vec::new();
    for p in pods.list(&lp).await? {
        ret.push(p);
    }
    Ok(ret)
}

/// Returns a set of ports that are used in a pod
pub async fn get_pod_ports(pod: NettingPod) -> HashSet<i32> {
    let mut ports: HashSet<i32> = HashSet::new();
    for container in pod.containers {
        for cp in container.ports {
            ports.insert(cp);
        }
    }
    ports
}
