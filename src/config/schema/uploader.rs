use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Uploader {
    pub file: Vec<File>,
    pub grpc: Vec<Grpc>,
    pub tcp: Vec<Tcp>,
    pub udp: Vec<Udp>,
}
impl Default for Uploader {
    fn default() -> Self {
        Uploader {
            file: Vec::new(),
            grpc: Vec::new(),
            tcp: Vec::new(),
            udp: Vec::new(),
        }
    }
}

fn wildcard() -> String {
    ".*".to_string()
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Tcp {
    pub enabled: bool,
    pub host: String,
    pub port: i64,
    #[serde(default = "wildcard")]
    pub pattern: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Grpc {
    pub enabled: bool,
    pub host: String,
    pub port: i64,
    #[serde(default = "wildcard")]
    pub pattern: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Udp {
    pub enabled: bool,
    pub host: String,
    pub port: i64,
    #[serde(default = "wildcard")]
    pub pattern: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct File {
    pub enabled: bool,
    pub path: String,
    #[serde(default = "wildcard")]
    pub pattern: String,
}
