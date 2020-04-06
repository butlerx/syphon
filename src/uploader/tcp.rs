use std::{
    error::Error,
    net::{ ToSocketAddrs, SocketAddr },
};
use tokio::{
    prelude::*,
    net::TcpStream,
    sync::broadcast::Receiver,
};
use regex::Regex;
use crate::parser::Metric;
use super::graphite;

pub async fn uploader(
    host: String,
    port: i64,
    pattern: String,
    mut rx: Receiver<Metric>
) -> Result<(), Box<dyn Error>> {
    let remote_addr:SocketAddr = format!("{}:{}", host, port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let re = Regex::new(&pattern).unwrap();
    let mut socket = TcpStream::connect(remote_addr).await?;
    info!(
        "conected to remote endpoint; proto={} remote_addr={} pattern={}",
        "tcp",
        remote_addr,
        pattern
    );
    loop {
        let res = rx.recv().await.unwrap() ;
        if re.is_match(res.path()) {
            socket.write_all(&graphite::format(res).into_bytes()).await?;
            debug!(
                "message sent; proto={} remote_addr={} pattern={}",
                "tcp",
                remote_addr,
                pattern
            );
        }
    }
}
