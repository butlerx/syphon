use std::{
    error::Error,
    net::{ ToSocketAddrs, SocketAddr },
};
use tokio::{
    net::UdpSocket,
    sync::broadcast::Receiver,
};
use crate::parser::Metric;
use super::graphite;

async fn uploader(addr: &String, rx: Receiver<Metric>) -> Result<(), Box<dyn Error>> {
    let remote_addr:SocketAddr = addr.to_socket_addrs().unwrap().next().unwrap();
    let local_addr:SocketAddr = if remote_addr.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    }
    .parse()?;

    let mut socket = UdpSocket::bind(local_addr).await?;
    socket.connect(&remote_addr).await?;
    while let res = rx.recv().await.unwrap() {
        socket.send(&graphite::format(res).into_bytes()).await?;
    }
    Ok(())
}
