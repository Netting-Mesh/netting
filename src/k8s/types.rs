#[derive(Debug, Clone)]
pub struct NettingContainer {
    pub name: String,
    pub image: String,
    pub ports: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct NettingPod {
    pub name: String,
    pub namespace: String,
    pub replicaset: String,
    pub containers: Vec<NettingContainer>,
    pub status: String,
    pub exposed: bool,
}

#[derive(Debug, Clone)]
pub struct NettingReplicaSet {
    pub name: String,
    pub namespace: String,
    pub deployment: String,
    pub pods: Vec<NettingPod>,
}

#[derive(Debug, Clone)]
pub struct NettingDeployment {
    pub name: String,
    pub namespace: String,
    pub replica_sets: Vec<NettingReplicaSet>,
}

#[derive(Debug, Clone)]
pub struct NettingService {
    pub name: String,
    pub namespace: String,
    pub cluster_ip: String,
    pub pods_exposed: Vec<NettingPod>,
}
