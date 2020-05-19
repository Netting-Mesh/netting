use crate::k8s::pod::{get_pod_details, get_pod_list};
use crate::k8s::types::*;
use k8s_openapi::api::apps::v1::{Deployment, ReplicaSet};
use kube::{
    api::{Api, ListParams},
    Client,
};

/// Returns a list of Deployment objects within
/// the given namespace filtered by the labels
pub async fn get_deployment_list(
    client: Client,
    namespace: String,
    labels: String,
) -> Result<Vec<Deployment>, kube::Error> {
    let deployments: Api<Deployment> = Api::namespaced(client, namespace.as_ref());
    let lp = ListParams::default().labels(labels.as_ref());
    let mut ret = Vec::new();
    for d in deployments.list(&lp).await? {
        ret.push(d);
    }
    Ok(ret)
}

/// Returns a list of ReplicaSet objects within
/// the given namespace filtered by the labels
pub async fn get_replicaset_list(
    client: Client,
    namespace: String,
    labels: String,
) -> Result<Vec<ReplicaSet>, kube::Error> {
    let replicasets: Api<ReplicaSet> = Api::namespaced(client, namespace.as_ref());
    let lp = ListParams::default().labels(labels.as_ref());
    let mut ret = Vec::new();
    for r in replicasets.list(&lp).await? {
        ret.push(r);
    }
    Ok(ret)
}

/// Returns a wrapper object for the given
/// ReplicaSet object
pub async fn get_replicaset_details(
    replicaset: ReplicaSet,
    client: Client,
) -> Result<NettingReplicaSet, kube::Error> {
    if let Some(metadata) = replicaset.metadata {
        let mut labels = String::new();
        if let Some(meta_labels) = metadata.labels {
            for (key, value) in meta_labels {
                labels.push_str(format!("{}={},", key, value).as_ref());
            }
            labels.pop(); // Remove trailing comma
        }
        let pods = get_pod_list(
            client,
            metadata
                .namespace
                .clone()
                .expect("Namespace undefined in replicaset metadata"),
            labels,
        )
        .await?;
        let mut netting_pods = Vec::new();
        for pod in pods {
            netting_pods.push(get_pod_details(pod, false).await?);
        }
        Ok(NettingReplicaSet {
            name: metadata
                .name
                .expect("Name undefined in replicaset metadata"),
            namespace: metadata
                .namespace
                .expect("Namespace undefined in replicaset metadata"),
            deployment: "".to_owned(),
            pods: netting_pods,
        })
    } else {
        return Err(kube::Error::Kubeconfig(
            "Replicaset metadata undefined".to_owned(),
        ));
    }
}

/// Returns a wrapper object for the given
/// Deployment object
pub async fn get_deployment_details(
    deploy: Deployment,
    client: Client,
) -> Result<NettingDeployment, kube::Error> {
    if let Some(spec) = deploy.spec {
        let mut labels = String::new();
        if let Some(spec_labels) = spec.selector.match_labels {
            for (key, value) in spec_labels {
                labels.push_str(format!("{}={},", key, value).as_ref());
            }
            labels.pop(); // Remove trailing comma
        }
        if let Some(metadata) = deploy.metadata {
            let replicasets = get_replicaset_list(
                client.clone(),
                metadata
                    .namespace
                    .clone()
                    .expect("Namespace undefined in deployment metadata"),
                labels,
            )
            .await?;
            let mut netting_replica_sets = Vec::new();
            for rs in replicasets {
                netting_replica_sets.push(get_replicaset_details(rs, client.clone()).await?);
            }
            Ok(NettingDeployment {
                name: metadata
                    .name
                    .expect("Name undefined in deployment metadata"),
                namespace: metadata
                    .namespace
                    .expect("Namespace undefined in deployment metadata"),
                replica_sets: netting_replica_sets,
            })
        } else {
            return Err(kube::Error::Kubeconfig(
                "Deployment metadata undefined".to_owned(),
            ));
        }
    } else {
        return Err(kube::Error::Kubeconfig(
            "Deployment spec undefined".to_owned(),
        ));
    }
}
