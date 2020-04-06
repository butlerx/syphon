use async_trait::async_trait;
use crate::parser::{graphite, Metric};
use std::io;
use super::base;
use tokio::{
    self,
    io::{AsyncReadExt},
    net::TcpListener,
    sync::broadcast::Sender,
};

static TCP_BUFFER_SIZE: usize = 1024;

pub struct Server {
    listener: TcpListener,
    sender: Sender<Metric>,
}

#[async_trait]
impl base::Receiver for Server {
    async fn bind(addr: &String, sender: Sender<Metric>) -> io::Result<Server> {
        let listener = TcpListener::bind(&addr).await?;
        Ok(Server{ listener, sender })
    }

    fn addr(&self) -> io::Result<String> {
        Ok(self.listener.local_addr().unwrap().to_string())
    }

    async fn run(&mut self) -> Result<(), io::Error> {
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
                    for metric in graphite::parse(msg) {
                        sender
                            .send(metric)
                            .expect("failed to write data to channel");
                    }
                    println!("Recieved {} bytes from {}", size, peer);
                }
            });
        }
    }
}
