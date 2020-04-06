use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod receiver;
mod uploader;

static METRIC_ENDPOINT_LOCAL: &str = "local";

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Config {
    #[serde(default)]
    pub metric: Metric,
    #[serde(default = "HashMap::new", serialize_with = "toml::ser::tables_last")]
    pub logging: HashMap<String, String>,
    #[serde(default)]
    pub file: receiver::File,
    #[serde(default)]
    pub udp: receiver::Udp,
    #[serde(default)]
    pub tcp: receiver::Tcp,
    #[serde(default)]
    pub prometheus: receiver::Prometheus,
    #[serde(default)]
    pub uploader: uploader::Uploader,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Metric {
    pub endpoint: String,
    pub interval: String,
}
impl Default for Metric {
    fn default() -> Self {
        Metric {
            endpoint: METRIC_ENDPOINT_LOCAL.to_string(),
            interval: "1m0s".to_string(),
        }
    }
}
