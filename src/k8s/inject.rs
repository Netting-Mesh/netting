use crate::k8s::pod::get_pod_ports;
use crate::k8s::types::*;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    api::{Api, PatchParams, PatchStrategy},
    Client,
};
use serde_json::json;
use std::collections::HashSet;

pub async fn inject_container_into_deploy(
    client: Client,
    deploy: NettingDeployment,
    container: NettingContainer,
) -> Result<(), kube::Error> {
    let deployments: Api<Deployment> = Api::namespaced(client, deploy.namespace.as_ref());
    let pp = PatchParams {
        force: true,
        patch_strategy: PatchStrategy::Apply,
        field_manager: Some("netting-field-manager".to_string()),
        ..Default::default()
    };
    let mut ports: HashSet<i32> = HashSet::new();
    for rs in deploy.replica_sets {
        for pod in rs.pods {
            ports.extend(get_pod_ports(pod).await);
        }
    }
    let patch = build_patch(container, ports).await;
    deployments
        .patch(
            deploy.name.as_ref(),
            &pp,
            serde_json::to_vec(&patch).unwrap(),
        )
        .await?;
    Ok(())
}

async fn build_patch(
    container: NettingContainer,
    port_to_redirect: HashSet<i32>,
) -> serde_json::value::Value {
    let mut cmd = String::new();
    for port in port_to_redirect {
        cmd.push_str(format!("iptables -t nat -A PREROUTING -p tcp -i eth0 --dport {:?} -j REDIRECT --to-port 50051;", port).as_ref());
    }
    let command = vec!["bash", "-c", cmd.as_ref()];
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
                            "containerPort": container.ports.get(0).expect("Container ports undefined"),
                            "protocol": "TCP",
                        }],
                    }],
                    "initContainers": [{
                        "name": "init-netting",
                        "image": "init-netting:v2",
                        "command": command,
                        "securityContext": {
                            "capabilities": {
                                "add": ["NET_ADMIN"]
                            },
                            "privileged": true,
                        }
                    }]
                }
            }
        }
    });
    patch
}
