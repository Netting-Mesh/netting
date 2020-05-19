use crate::k8s::deploy::{get_deployment_details, get_deployment_list};
use crate::k8s::namespace::get_namespaces;
use crate::k8s::pod::{get_pod_details, get_pod_list};
use crate::k8s::svc::{get_svc_details, get_svc_list};
use crate::k8s::types::*;
use kube::Client;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Catalog {
    pub pod_catalog: HashMap<String, Vec<NettingPod>>,
    pub deploy_catalog: HashMap<String, Vec<NettingDeployment>>,
    pub service_catalog: HashMap<String, Vec<NettingService>>,
    pub namespaces: Vec<String>,
}

impl Catalog {
    pub async fn new() -> Self {
        Catalog {
            pod_catalog: HashMap::default(),
            deploy_catalog: HashMap::default(),
            service_catalog: HashMap::default(),
            namespaces: Vec::default(),
        }
    }
    pub async fn build_catalog(&mut self, client: Client) {
        let namespaces = get_namespaces(client.clone()).await.unwrap();
        self.pod_catalog = self
            .build_pod_catalog(client.clone(), namespaces.clone())
            .await;
        self.deploy_catalog = self
            .build_deploy_catalog(client.clone(), namespaces.clone())
            .await;
        self.service_catalog = self
            .build_svc_catalog(client.clone(), namespaces.clone())
            .await;
        self.namespaces = namespaces;
    }
    pub async fn clear_catalog(&mut self) {
        self.namespaces.clear();
        self.pod_catalog.clear();
        self.deploy_catalog.clear();
        self.service_catalog.clear();
    }
    async fn build_pod_catalog(
        &mut self,
        client: Client,
        namespaces: Vec<String>,
    ) -> HashMap<String, Vec<NettingPod>> {
        let mut pod_catalog: HashMap<String, Vec<NettingPod>> = HashMap::new();
        for namespace in namespaces {
            let pods = get_pod_list(client.clone(), namespace.clone(), "".to_owned()).await;
            for pod in pods.unwrap() {
                match get_pod_details(pod, false).await {
                    Ok(netting_pod) => match pod_catalog.get_mut(&(namespace.clone())) {
                        Some(pods) => {
                            pods.push(netting_pod);
                        }
                        None => {
                            pod_catalog.insert(namespace.clone(), vec![netting_pod]);
                        }
                    },
                    Err(err) => println!("{}", err),
                }
            }
        }
        pod_catalog
    }
    async fn build_deploy_catalog(
        &mut self,
        client: Client,
        namespaces: Vec<String>,
    ) -> HashMap<String, Vec<NettingDeployment>> {
        let mut deploy_catalog: HashMap<String, Vec<NettingDeployment>> = HashMap::new();
        for namespace in namespaces {
            let deployments =
                get_deployment_list(client.clone(), namespace.clone(), "".to_owned()).await;
            for deploy in deployments.unwrap() {
                match get_deployment_details(deploy, client.clone()).await {
                    Ok(netting_deploy) => match deploy_catalog.get_mut(&(namespace.clone())) {
                        Some(deployments) => {
                            deployments.push(netting_deploy);
                        }
                        None => {
                            deploy_catalog.insert(namespace.clone(), vec![netting_deploy]);
                        }
                    },
                    Err(err) => println!("{}", err),
                }
            }
        }
        deploy_catalog
    }
    async fn update_pod(&mut self, namespace: String, pod: NettingPod) {
        match self.pod_catalog.get_mut(&namespace) {
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
    async fn build_svc_catalog(
        &mut self,
        client: Client,
        namespaces: Vec<String>,
    ) -> HashMap<String, Vec<NettingService>> {
        let mut svc_catalog: HashMap<String, Vec<NettingService>> = HashMap::new();
        for namespace in namespaces {
            match get_svc_list(client.clone(), "".to_owned(), namespace.clone()).await {
                Ok(services) => {
                    for svc in services {
                        match get_svc_details(svc, client.clone()).await {
                            Ok(netting_service) => {
                                match svc_catalog.get_mut(&(namespace.clone())) {
                                    Some(services) => {
                                        services.push(netting_service.clone());
                                        for pod in netting_service.pods_exposed {
                                            self.update_pod(namespace.clone(), pod).await;
                                        }
                                    }
                                    None => {
                                        svc_catalog.insert(
                                            namespace.clone(),
                                            vec![netting_service.clone()],
                                        );
                                        for pod in netting_service.pods_exposed {
                                            self.update_pod(namespace.clone(), pod).await;
                                        }
                                    }
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
        svc_catalog
    }
}
