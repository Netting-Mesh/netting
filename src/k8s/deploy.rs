use crate::k8s::pod::{get_pod_details, get_pod_list};
use crate::k8s::types::*;
use k8s_openapi::api::apps::v1::{Deployment, ReplicaSet};
use kube::{
    api::{Api, ListParams, PatchParams, PatchStrategy},
    Client,
};
use serde_json::json;

pub async fn get_deployment_list(
    client: Client,
    namespace: String,
    labels: String,
) -> Result<Vec<Deployment>, kube::Error> {
    let deployments: Api<Deployment> = Api::namespaced(client, namespace.as_ref());
    let lp = ListParams::default().labels("");
    let mut ret = Vec::new();
    for d in deployments.list(&lp).await? {
        ret.push(d);
    }
    Ok(ret)
}

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

pub async fn get_replicaset_details(replicaset: ReplicaSet, client: Client) -> NettingReplicaSet {
    let mut labels = String::new();
    for (key, value) in replicaset.metadata.clone().unwrap().labels.unwrap() {
        labels.push_str(format!("{}={},", key, value).as_ref());
    }
    labels.pop();
    let pods = get_pod_list(
        client,
        replicaset.metadata.clone().unwrap().namespace.unwrap(),
        labels,
    )
    .await;
    let mut netting_pods = Vec::new();
    for pod in pods.unwrap() {
        netting_pods.push(get_pod_details(pod, false).await);
    }
    return NettingReplicaSet {
        name: replicaset.metadata.clone().unwrap().name.unwrap(),
        namespace: replicaset.metadata.unwrap().namespace.unwrap(),
        deployment: "".to_owned(),
        pods: netting_pods,
    };
}

pub async fn get_deployment_details(deploy: Deployment, client: Client) -> NettingDeployment {
    let mut labels = String::new();
    for (key, value) in deploy.spec.clone().unwrap().selector.match_labels.unwrap() {
        labels.push_str(format!("{}={},", key, value).as_ref());
    }
    labels.pop();
    let replicasets = get_replicaset_list(
        client.clone(),
        deploy.metadata.clone().unwrap().namespace.unwrap(),
        labels,
    )
    .await;
    let mut netting_replica_sets = Vec::new();
    for rs in replicasets.unwrap() {
        netting_replica_sets.push(get_replicaset_details(rs, client.clone()).await);
    }
    return NettingDeployment {
        name: deploy.metadata.clone().unwrap().name.unwrap(),
        namespace: deploy.metadata.clone().unwrap().namespace.unwrap(),
        replica_sets: netting_replica_sets,
    };
}
