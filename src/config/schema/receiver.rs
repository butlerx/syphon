use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct File {
    pub enabled: bool,
    pub path: String,
}
impl Default for File {
    fn default() -> Self {
        File {
            path: "metrics.txt".to_string(),
            enabled: false,
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Udp {
    pub enabled: bool,
    pub listen: String,
}
impl Default for Udp {
    fn default() -> Self {
        Udp {
            listen: "127.0.0.1:2003".to_string(),
            enabled: true,
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Tcp {
    pub enabled: bool,
    pub listen: String,
}
impl Default for Tcp {
    fn default() -> Self {
        Tcp {
            listen: "127.0.0.1:2003".to_string(),
            enabled: true,
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Prometheus {
    pub enabled: bool,
    pub listen: String,
}
impl Default for Prometheus {
    fn default() -> Self {
        Prometheus {
            listen: "127.0.0.1:2006".to_string(),
            enabled: false,
        }
    }
}
