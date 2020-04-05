use std::{
    error::Error,
    net::{ ToSocketAddrs, SocketAddr },
};
use tokio::{
    prelude::*,
    net::TcpStream,
    sync::broadcast::Receiver,
};
use crate::parser::Metric;
use super::graphite;

async fn uploader(addr: String, rx: Receiver<Metric>) -> Result<(), Box<dyn Error>> {
    let remote_addr:SocketAddr = addr.to_socket_addrs().unwrap().next().unwrap();

    let mut socket = TcpStream::connect(remote_addr).await?;
    while let res = rx.recv().await.unwrap() {
        socket.write_all(&graphite::format(res).into_bytes()).await?;
    }
    Ok(())
}
