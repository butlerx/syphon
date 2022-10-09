use super::graphite;
use crate::parser::Metric;
use kv_log_macro as log;
use regex::Regex;
use std::{
    error::Error,
    net::{SocketAddr, ToSocketAddrs},
};
use tokio::{net::UdpSocket, sync::broadcast::Receiver};

pub async fn uploader(
    host: String,
    port: i64,
    pattern: String,
    mut rx: Receiver<Metric>,
) -> Result<(), Box<dyn Error>> {
    let remote_addr_str = format!("{}:{}", host, port);
    let remote_addr: SocketAddr = remote_addr_str.to_socket_addrs().unwrap().next().unwrap();
    let local_addr_str = if remote_addr.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    };
    let local_addr: SocketAddr = local_addr_str.parse()?;

    let re = Regex::new(&pattern).unwrap();
    let mut socket = UdpSocket::bind(local_addr).await?;
    socket.connect(&remote_addr).await?;
    log::info!(
        "conected to remote endpoint",
        {proto: "udp", remote_addr: remote_addr_str, addr: local_addr_str, patter: pattern}
    );
    loop {
        let res = rx.recv().await.unwrap();
        if re.is_match(res.path()) {
            socket.send(&graphite::format(res).into_bytes()).await?;
            log::debug!(
                "message sent",
                {proto: "udp", remote_addr: remote_addr_str, addr: local_addr_str, patter: pattern}
            );
        }
    }
}
