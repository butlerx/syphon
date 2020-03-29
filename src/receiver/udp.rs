use std::net::SocketAddr;
use std::{io};
use tokio;
use tokio::net::UdpSocket;
use tokio::sync::broadcast::Sender;

static UDP_BUFFER_SIZE: usize = 1024;

pub struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
    sender: Sender<String>,
}

impl Server {
    pub async fn bind(addr: &String, sender: Sender<String>) -> io::Result<Server> {
        let socket = UdpSocket::bind(&addr).await?;
        let server = Server{
            socket,
            buf: vec![0; UDP_BUFFER_SIZE],
            to_send: None,
            sender,
        };
        Ok(server)
    }

    pub fn addr(&self) -> io::Result<std::net::SocketAddr> {
        self.socket.local_addr()
    }

    pub async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            if let Some((size, peer)) = self.to_send {
                let msg = String::from_utf8_lossy(&self.buf[0..size]).to_string();
                self
                    .sender
                    .send(msg)
                    .expect("failed to write data to channel");
                println!("Recieved {} bytes from {}", size, peer);
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to recieve
            self.to_send = Some(self.socket.recv_from(&mut self.buf).await?);
        }
    }
}
