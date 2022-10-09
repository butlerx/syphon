use crate::parser;
use kv_log_macro as log;
use regex::Regex;
use std::error::Error;
use tokio::sync::broadcast::Receiver;

pub mod carbon {
    tonic::include_proto!("carbon");
}

use carbon::{carbon_client::CarbonClient, Metric, Payload, Point};

pub async fn uploader(
    host: String,
    port: i64,
    pattern: String,
    mut rx: Receiver<parser::Metric>,
) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(&pattern).unwrap();
    let remote_addr = format!("{}:{}", host, port);
    let mut client = CarbonClient::connect(remote_addr.clone()).await?;
    log::info!(
        "conected to remote endpoint",{
        proto: "grpc", remote_addr: remote_addr, patter: pattern}
    );
    loop {
        let res = rx.recv().await.unwrap();
        if re.is_match(res.path()) {
            let request = tonic::Request::new(Payload {
                metrics: vec![Metric {
                    metric: res.path().into(),
                    points: vec![Point {
                        timestamp: *res.time() as u32,
                        value: *res.value() as f64,
                    }],
                }],
            });
            let _ = client.store(request).await?;
            log::debug!(
                "message sent",{
                proto: "grpc", remote_addr: remote_addr, patter: pattern}
            );
        }
    }
}
