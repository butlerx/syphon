use tokio;
use tokio::io::{AsyncReadExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast::Sender;
use std::{io};


static TCP_BUFFER_SIZE: usize = 1024;

pub struct Server {
    listener: TcpListener,
    sender: Sender<String>,
}

impl Server {
    pub async fn bind(addr: &String, sender: Sender<String>) -> io::Result<Server> {
        let listener = TcpListener::bind(&addr).await?;
        let server = Server{listener, sender};
        Ok(server)
    }

    pub fn addr(&self) -> io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }

    pub async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            let (mut socket, peer) = self.listener.accept().await?;
            let sender = self.sender.clone();
            tokio::spawn(async move {
                let mut buf = vec![0; TCP_BUFFER_SIZE];
                loop {
                    let size = socket
                        .read(&mut buf)
                        .await
                        .expect("failed to read data from socket");

                    if size == 0 {
                        return;
                    }
                    let msg = String::from_utf8_lossy(&buf[0..size]).to_string();
                    sender
                        .send(msg)
                        .expect("failed to write data to channel");
                    println!("Recieved {} bytes from {}", size, peer);
                }
            });
        }
    }
}
