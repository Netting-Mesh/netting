use crate::k8s::deploy::{get_deployment_details, get_deployment_list};
use crate::k8s::namespace::get_namespaces;
use crate::k8s::pod::{get_pod_details, get_pod_list};
use crate::k8s::svc::{get_svc_details, get_svc_list};
use crate::k8s::types::*;
use kube::Client;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Catalogue {
    pub pod_catalogue: HashMap<String, Vec<NettingPod>>,
    pub deploy_catalogue: HashMap<String, Vec<NettingDeployment>>,
    pub service_catalogue: HashMap<String, Vec<NettingService>>,
    pub namespaces: Vec<String>,
}

impl Catalogue {
    pub async fn new() -> Self {
        Catalogue {
            pod_catalogue: HashMap::default(),
            deploy_catalogue: HashMap::default(),
            service_catalogue: HashMap::default(),
            namespaces: Vec::default(),
        }
    }
    pub async fn build_catalogue(&mut self, client: Client) {
        let namespaces = get_namespaces(client.clone()).await.unwrap();
        self.pod_catalogue = self
            .build_pod_catalogue(client.clone(), namespaces.clone())
            .await;
        self.deploy_catalogue = self
            .build_deploy_catalogue(client.clone(), namespaces.clone())
            .await;
        self.service_catalogue = self
            .build_svc_catalogue(client.clone(), namespaces.clone())
            .await;
        self.namespaces = namespaces;
    }
    pub async fn clear_catalogue(&mut self) {
        self.namespaces.clear();
        self.pod_catalogue.clear();
        self.deploy_catalogue.clear();
        self.service_catalogue.clear();
    }
    async fn build_pod_catalogue(
        &mut self,
        client: Client,
        namespaces: Vec<String>,
    ) -> HashMap<String, Vec<NettingPod>> {
        let mut pod_catalogue: HashMap<String, Vec<NettingPod>> = HashMap::new();
        for namespace in namespaces {
            let pods = get_pod_list(client.clone(), namespace.clone(), "".to_owned()).await;
            for pod in pods.unwrap() {
                let netting_pod = get_pod_details(pod, false).await;
                match pod_catalogue.get_mut(&(namespace.clone())) {
                    Some(pods) => {
                        pods.push(netting_pod);
                    }
                    None => {
                        pod_catalogue.insert(namespace.clone(), vec![netting_pod]);
                    }
                }
            }
        }
        pod_catalogue
    }
    async fn build_deploy_catalogue(
        &mut self,
        client: Client,
        namespaces: Vec<String>,
    ) -> HashMap<String, Vec<NettingDeployment>> {
        let mut deploy_catalogue: HashMap<String, Vec<NettingDeployment>> = HashMap::new();
        for namespace in namespaces {
            let deployments =
                get_deployment_list(client.clone(), namespace.clone(), "".to_owned()).await;
            for deploy in deployments.unwrap() {
                let netting_deploy = get_deployment_details(deploy, client.clone()).await;
                match deploy_catalogue.get_mut(&(namespace.clone())) {
                    Some(deployments) => {
                        deployments.push(netting_deploy);
                    }
                    None => {
                        deploy_catalogue.insert(namespace.clone(), vec![netting_deploy]);
                    }
                }
            }
        }
        deploy_catalogue
    }
    async fn update_pod(&mut self, namespace: String, pod: NettingPod) {
        match self.pod_catalogue.get_mut(&namespace) {
            Some(pods) => {
                for p in pods {
                    if pod.name == p.name {
                        p.exposed = true;
                    }
                }
            }
            None => {}
        }
    }
    async fn build_svc_catalogue(
        &mut self,
        client: Client,
        namespaces: Vec<String>,
    ) -> HashMap<String, Vec<NettingService>> {
        let mut svc_catalogue: HashMap<String, Vec<NettingService>> = HashMap::new();
        for namespace in namespaces {
            let services = get_svc_list(client.clone(), namespace.clone()).await;
            for svc in services.unwrap() {
                match get_svc_details(svc, client.clone()).await {
                    Ok(netting_service) => match svc_catalogue.get_mut(&(namespace.clone())) {
                        Some(services) => {
                            services.push(netting_service.clone());
                            for pod in netting_service.pods_exposed {
                                self.update_pod(namespace.clone(), pod).await;
                            }
                        }
                        None => {
                            svc_catalogue.insert(namespace.clone(), vec![netting_service.clone()]);
                            for pod in netting_service.pods_exposed {
                                self.update_pod(namespace.clone(), pod).await;
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
        }
        svc_catalogue
    }
}
