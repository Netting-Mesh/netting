use crate::k8s::namespace::get_namespaces;
use crate::k8s::pod::{get_pod_details, get_pod_list};
use crate::k8s::types::*;
use kube::Client;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Catalogue {
    pub pod_catalogue: HashMap<String, Vec<NettingPod>>,
    //pub deploy_catalogue: HashMap<String, Vec<NettingDeployment>>,
    //pub service_catalogue: HashMap<String, Vec<NettingService>>,
    pub namespaces: Vec<String>,
}

impl Catalogue {
    pub async fn new(client: Client) -> Self {
        let namespaces = get_namespaces(client.clone()).await.unwrap();
        return Catalogue {
            pod_catalogue: Catalogue::build_pod_catalogue(client, namespaces.clone()).await,
            namespaces: namespaces,
        };
    }
    pub async fn build_pod_catalogue(
        client: Client,
        namespaces: Vec<String>,
    ) -> HashMap<String, Vec<NettingPod>> {
        let mut pod_catalogue: HashMap<String, Vec<NettingPod>> = HashMap::new();
        for namespace in namespaces {
            let pods = get_pod_list(client.clone(), namespace.clone(), "".to_owned()).await;
            for pod in pods.unwrap() {
                let ns = namespace.clone();
                let netting_pod = get_pod_details(pod).await;
                match pod_catalogue.get_mut(&ns) {
                    Some(pods) => {
                        pods.push(netting_pod);
                    }
                    None => {
                        pod_catalogue.insert(ns, vec![netting_pod]);
                    }
                }
            }
        }
        pod_catalogue
    }
}
