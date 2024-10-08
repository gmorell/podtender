use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GetInfoResponse {
    pub host: Option<HostInfo>,
    pub plugins: Option<Plugins>,
    pub registries: Option<HashMap<String, Vec<String>>>,
    pub store: Option<StoreInfo>,
    pub version: Option<Version>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NetDNS {
    pub package: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NetworkBackendInfo {
    pub backend: Option<String>,
    pub version: Option<String>,
    pub package: Option<String>,
    pub path: Option<String>,
    pub dns: Option<NetDNS>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pasta {
    pub executable: Option<String>,
    pub version: Option<String>,
    pub package: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct HostInfo {
    pub arch: Option<String>,
    pub buildah_version: Option<String>,
    pub cgroup_controllers: Option<Vec<String>>,
    pub cgroup_manager: Option<String>,
    pub cgroup_version: Option<String>,
    pub conmon: Option<ConmonInfo>,
    pub cpus: Option<i64>,
    pub cpu_utilization: Option<CpuUsage>,
    pub database_backend: Option<String>,
    pub distribution: Option<DistributionInfo>,
    pub event_logger: Option<String>,
    pub free_locks: Option<i64>,
    pub hostname: Option<String>,
    pub id_mappings: Option<IdMappings>,
    pub kernel: Option<String>,
    pub linkmode: Option<String>,
    pub log_driver: Option<String>,
    pub mem_free: Option<i64>,
    pub mem_total: Option<i64>,
    pub network_backend: Option<String>,
    pub network_backend_info: Option<NetworkBackendInfo>,
    pub oci_runtime: Option<OciRuntime>,
    pub os: Option<String>,
    pub pasta: Option<Pasta>,
    pub remote_socket: Option<RemoteSocket>,
    pub rootless_network_cmd: Option<String>,
    pub runtime_info: Option<HashMap<String, String>>,
    pub security: Option<SecurityInfo>,
    pub service_is_remote: Option<bool>,
    pub slirp4netns: Option<SlirpInfo>,
    pub swap_free: Option<i64>,
    pub swap_total: Option<i64>,
    pub uptime: Option<String>,
    pub variant: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConmonInfo {
    pub package: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CpuUsage {
    pub idle_percent: Option<f32>,
    pub system_percent: Option<f32>,
    pub user_percent: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DistributionInfo {
    pub codename: Option<String>,
    pub distribution: Option<String>,
    pub variant: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IdMappings {
    pub gidmap: Option<Vec<IdMap>>,
    pub uidmap: Option<Vec<IdMap>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IdMap {
    pub container_id: Option<i64>,
    pub host_id: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OciRuntime {
    pub name: Option<String>,
    pub package: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RemoteSocket {
    #[serde(default = "RemoteSocket::default_exists")]
    pub exists: bool,
    pub path: Option<String>,
}
impl RemoteSocket {
    fn default_exists() -> bool {
        false
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SecurityInfo {
    pub apparmor_enabled: Option<bool>,
    pub capabilities: Option<String>,
    pub rootless: Option<bool>,
    pub seccomp_enabled: Option<bool>,
    pub seccomp_profile_path: Option<String>,
    pub selinux_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SlirpInfo {
    pub executable: Option<String>,
    pub package: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Plugins {
    pub authorization: Option<Vec<String>>,
    pub log: Option<Vec<String>>,
    pub network: Option<Vec<String>>,
    pub volume: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StoreInfo {
    pub config_file: Option<String>,
    pub container_store: Option<ContainerStore>,
    pub graph_driver_name: Option<String>,
    pub graph_options: Option<HashMap<String, String>>,
    pub graph_root: Option<String>,
    pub graph_root_allocated: Option<u64>,
    pub graph_root_used: Option<u64>,
    pub graph_status: Option<HashMap<String, String>>,
    pub image_copy_tmp_dir: Option<String>,
    pub image_store: Option<ImageStore>,
    pub run_root: Option<String>,
    pub transient_store: Option<bool>,
    pub volume_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContainerStore {
    pub number: Option<i64>,
    pub paused: Option<i64>,
    pub running: Option<i64>,
    pub stopped: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImageStore {
    pub number: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    #[serde(rename = "APIVersion")]
    pub api_version: Option<String>,
    pub built: Option<i64>,
    pub built_time: Option<String>,
    pub git_commit: Option<String>,
    pub go_version: Option<String>,
    pub os: Option<String>,
    pub os_arch: Option<String>,
    pub version: Option<String>,
    pub index: Option<i64>,
}

// Info taken from https://github.com/containers/podman/blob/69085570f7ebbb3768e963e2a6a31d7bb9b4ca16/libpod/info.go#L265-L281
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct GraphOptionsEntry {
    pub executable: Option<String>,
    pub package: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub status: Option<String>,
    pub id: Option<String>,
    pub from: Option<String>,
    #[serde(rename = "Type")]
    pub event_type: Option<String>,
    #[serde(rename = "Action")]
    pub action: Option<String>,
    #[serde(rename = "Actor")]
    pub actor: EventActor,
    pub scope: Option<String>,
    pub time: Option<u64>,
    #[serde(rename = "timeNano")]
    pub time_nano: Option<u64>,
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EventActor {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "Attributes")]
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DfImage {
    #[serde(rename = "Repository")]
    pub repository: String,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Size")]
    pub size: u64,
    #[serde(rename = "SharedSize")]
    pub shared_size: u64,
    #[serde(rename = "UniqueSize")]
    pub inique_size: u64,
    #[serde(rename = "Containers")]
    pub containers: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DfContainer {
    #[serde(rename = "ContainerID")]
    pub container_id: String,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Command")]
    pub command: Vec<String>,
    #[serde(rename = "LocalVolumes")]
    pub local_volumes: u64,
    #[serde(rename = "Size")]
    pub size: u64,
    #[serde(rename = "RWSize")]
    pub rw_size: u64,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Names")]
    pub names: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DfVolume {
    #[serde(rename = "Links")]
    pub links: u64,
    #[serde(rename = "ReclaimableSize")]
    pub reclaimable_size: u64,
    #[serde(rename = "Size")]
    pub size: u64,
    #[serde(rename = "VolumeName")]
    pub volume_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DfResponse {
    #[serde(rename = "ImagesSize")]
    pub images_size: u64,
    #[serde(rename = "Images")]
    pub images: Vec<DfImage>,
    #[serde(rename = "Containers")]
    pub containers: Vec<DfContainer>,
    #[serde(rename = "Volumes")]
    pub volumes: Vec<DfVolume>,
}
