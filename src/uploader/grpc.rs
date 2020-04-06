use std::error::Error;
use tokio::sync::broadcast::Receiver;
use regex::Regex;
use crate::parser;
use carbon::carbon_client::CarbonClient;
use carbon::{Payload, Metric, Point};

pub mod carbon {
    tonic::include_proto!("carbon");
}

pub async fn uploader(
    host: String,
    port: i64,
    pattern: String,
    mut rx: Receiver<parser::Metric>
) -> Result<(), Box<dyn Error>> {

    let re = Regex::new(&pattern).unwrap();
    let mut client = CarbonClient::connect(format!("{}:{}", host, port)).await?;
    info!(
        "conected to remote endpoint; proto={} remote_addr={} pattern={}",
        "grpc",
        format!("{}:{}", host, port),
        pattern
    );
    loop {
        let res = rx.recv().await.unwrap() ;
        if re.is_match(res.path()) {
            let request = tonic::Request::new(Payload {
                metrics: vec!(Metric {
                    metric: res.path().into(),
                    points: vec!(Point {
                        timestamp: *res.time() as u32,
                        value: *res.value() as f64,
                    })
                })
            });
            let _ = client.store(request).await?;
            debug!(
                "message sent; proto={} remote_addr={} pattern={}",
                "grpc",
                format!("{}:{}", host, port),
                pattern
            );
        }
    }
}
