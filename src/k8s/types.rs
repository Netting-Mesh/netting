use std::collections::HashMap;

#[derive(Debug)]
pub struct NettingContainer {
    pub image: String,
    pub ports: Vec<i32>,
    pub running: bool,
}

#[derive(Debug)]
pub struct NettingPod {
    pub name: String,
    pub namespace: String,
    pub replicaset: String,
    pub containers: Vec<NettingContainer>,
    pub status: String,
    pub exposed: bool,
}

#[derive(Debug)]
pub struct NettingReplicaSet {
    pub name: String,
    pub namespace: String,
    pub deployment: String,
    pub pods: Vec<NettingPod>,
}

#[derive(Debug)]
pub struct NettingDeployment {
    pub name: String,
    pub namespace: String,
    pub replicasets: Vec<NettingReplicaSet>,
}

#[derive(Debug)]
pub struct NettingService {
    pub name: String,
    pub namespace: String,
    pub clusterIp: String,
    pub podsExposed: Vec<NettingPod>,
}
