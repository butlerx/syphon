use std::collections::HashMap;

pub struct PrometheusLabel {
    name: Vec<u8>,
    value: Vec<u8>,
}

pub struct MetricBuffer {
    labels: Vec<PrometheusLabel>,
    queryEscape: HashMap<String, String>,
    metric: Vec<String>, // ["name", "key1", "value1", ...]
    metricUsed: i64,
}
impl MetricBuffer {
    pub fn new() -> MetricBuffer {
        MetricBuffer {
            labels: Vec::with_capacity(16),
            queryEscape: HashMap::new(),
            metric: Vec::with_capacity(128),
            metricUsed: 0,
        }
    }
}
