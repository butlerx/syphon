use crate::parser::{graphite, Metric};
use kv_log_macro as log;
use std::io;
use tokio::{self, io::AsyncReadExt, net::TcpListener, sync::broadcast::Sender};
static TCP_BUFFER_SIZE: usize = 1024;

pub async fn bind(addr: String, sender: Sender<Metric>) -> Result<(), io::Error> {
    let mut listener = TcpListener::bind(&addr).await?;
    log::info!( "Reciever listening", { proto: "tcp", addr: addr });

    loop {
        let (mut socket, peer) = listener.accept().await?;
        let sender = sender.clone();
        tokio::spawn(async move {
            let mut buf = vec![0; TCP_BUFFER_SIZE];
            let local_addr = socket.local_addr().unwrap().to_string();
            loop {
                let size = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if size == 0 {
                    return;
                }
                let msg = String::from_utf8_lossy(&buf[0..size]).to_string();
                for metric in graphite::parse(msg) {
                    sender
                        .send(metric)
                        .expect("failed to write data to channel");
                }
                log::debug!(
                "Recieved message", {
                proto: "tcp",
                bytes: size,
                remote_addr : peer.to_string(),
                addr: local_addr,
                });
            }
        });
    }
}
