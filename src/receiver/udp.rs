use std::{io, net::SocketAddr};
use tokio::{self, net::UdpSocket, sync::broadcast::Sender};
use crate::parser::{graphite, Metric};

static UDP_BUFFER_SIZE: usize = 1024;


pub async fn bind(addr: String, sender: Sender<Metric>) -> Result<(), io::Error> {
    let mut socket = UdpSocket::bind(&addr).await?;
    let mut buf: Vec<u8> = vec![0; UDP_BUFFER_SIZE];
    let mut to_send: Option<(usize, SocketAddr)> = None;
    info!(
        "Reciever listening; proto={} addr={}",
        "udp",
        socket.local_addr().unwrap()
    );

    loop {
        if let Some((size, peer)) = to_send {
            let msg = String::from_utf8_lossy(&buf[0..size]).to_string();
            for metric in graphite::parse(msg) {
                sender
                    .send(metric)
                    .expect("failed to write data to channel");
            }
            debug!(
                "Recieved message; proto={} bytes={} remote_addr={} local_addr={}",
                "udp",
                size,
                peer,
                socket.local_addr().unwrap()
            );
        }
        to_send = Some(socket.recv_from(&mut buf).await?);
    }
}
