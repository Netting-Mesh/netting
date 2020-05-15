use crate::k8s::types::*;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    api::{Api, PatchParams, PatchStrategy},
    Client,
};
use serde_json::json;

pub async fn inject_container_into_deploy(
    client: Client,
    deploy: NettingDeployment,
    container: NettingContainer,
) {
    let deployments: Api<Deployment> = Api::namespaced(client, deploy.namespace.as_ref());
    let pp = PatchParams {
        force: true,
        patch_strategy: PatchStrategy::Apply,
        field_manager: Some("netting-field-manager".to_string()),
        ..Default::default()
    };
    let patch = build_patch(container).await;
    let res = deployments
        .patch(
            deploy.name.as_ref(),
            &pp,
            serde_json::to_vec(&patch).unwrap(),
        )
        .await;
    println!("{:?}", res);
}

async fn build_patch(container: NettingContainer) -> serde_json::value::Value {
    let patch = json!({
    "apiVersion": "apps/v1",
    "kind": "Deployment",
    "spec": {
        "template": {
            "spec": {
                "containers": [{
                    "name": container.name,
                    "image": container.image,
                    "ports": [{
                        "containerPort": container.ports.get(0).unwrap(),
                        "protocol": "TCP",
                        }],
                    }],
                }
            }
        }
    });
    patch
}
