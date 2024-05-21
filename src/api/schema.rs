use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct EmbeddedParams {
    pub container_manager: String,
    pub home: String,
    pub container_id: String,
    pub log_level: String,
}

#[derive(Serialize, Deserialize)]
pub struct OCIConfig {
    #[serde(rename = "ociVersion")]
    pub oci_version: String,

    #[serde(rename = "process")]
    pub process: Process,
}

#[derive(Serialize, Deserialize)]
pub struct Process {
    #[serde(rename = "args")]
    pub args: Vec<String>,

    #[serde(rename = "env")]
    pub env: Vec<String>,
}
