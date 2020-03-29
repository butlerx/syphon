use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Uploader {
    pub file: Vec<File>,
    pub grpc: Vec<Grpc>,
    pub tcp: Vec<Tcp>,
    pub udp: Vec<Udp>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Tcp {
    pub enabled: bool,
    pub host: String,
    pub port: i64,
    pub pattern: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Grpc {
    pub enabled: bool,
    pub host: String,
    pub port: i64,
    pub pattern: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Udp {
    pub enabled: bool,
    pub host: String,
    pub port: i64,
    pub pattern: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct File {
    pub enabled: bool,
    pub path: String,
    pub pattern: String,
}
