use crate::k8s::types::*;
use k8s_openapi::api::core::v1::{Container, ContainerStatus, Pod};
use kube::{
    api::{Api, ListParams},
    Client,
};

pub async fn get_container_details(container: Container) -> NettingContainer {
    let mut ports = Vec::new();
    match container.ports {
        Some(container_ports) => {
            for cp in container_ports {
                ports.push(cp.container_port);
            }
        }
        None => {}
    }
    return NettingContainer {
        image: container.image.unwrap(),
        ports: ports,
    };
}

pub async fn get_container_status(status: ContainerStatus) -> bool {
    return status.ready;
}

pub async fn get_pod_details(pod: Pod) -> NettingPod {
    let mut containers = Vec::new();
    for container in pod.spec.unwrap().containers {
        containers.push(get_container_details(container).await);
    }
    let mut status = String::new();
    for condition in pod.status.unwrap().conditions.unwrap() {
        if condition.type_ == "Ready" {
            status = condition.status;
        }
    }
    return NettingPod {
        name: pod.metadata.clone().unwrap().name.unwrap(),
        namespace: pod.metadata.unwrap().namespace.unwrap(),
        replicaset: "".to_owned(),
        containers: containers,
        status: status,
        exposed: false,
    };
}

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
