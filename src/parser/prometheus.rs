use super::Metric;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn parse(msg: Vec<u8>) -> Vec<Metric> {
    println!("{:?}", msg);
    vec![Metric::new(
        "test".to_string(),
        HashMap::new(),
        0.0,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
    )]
}
