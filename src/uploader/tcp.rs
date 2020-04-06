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

pub async fn uploader(host: String, port: i64, mut rx: Receiver<Metric>) -> Result<(), Box<dyn Error>> {
    let remote_addr:SocketAddr = format!("{}:{}", host, port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let mut socket = TcpStream::connect(remote_addr).await?;
    loop {
        let res = rx.recv().await.unwrap() ;
        socket.write_all(&graphite::format(res).into_bytes()).await?;
    }
}
