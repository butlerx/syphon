use super::graphite;
use crate::parser::Metric;
use kv_log_macro as log;
use regex::Regex;
use std::{
    error::Error,
    net::{SocketAddr, ToSocketAddrs},
};
use tokio::{net::TcpStream, prelude::*, sync::broadcast::Receiver};

pub async fn uploader(
    host: String,
    port: i64,
    pattern: String,
    mut rx: Receiver<Metric>,
) -> Result<(), Box<dyn Error>> {
    let remote_addr_str = format!("{}:{}", host, port);
    let remote_addr: SocketAddr = remote_addr_str.to_socket_addrs().unwrap().next().unwrap();

    let re = Regex::new(&pattern).unwrap();
    let mut socket = TcpStream::connect(remote_addr).await?;
    log::info!(
        "conected to remote endpoint",
        { proto: "tcp", remote_addr:remote_addr_str, pattern: pattern}
    );
    loop {
        let res = rx.recv().await.unwrap();
        if re.is_match(res.path()) {
            socket
                .write_all(&graphite::format(res).into_bytes())
                .await?;
            log::debug!( "message sent", { proto: "tcp", remote_addr:remote_addr_str, pattern: pattern});
        }
    }
}
