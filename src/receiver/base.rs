use std::io;
use async_trait::async_trait;
use tokio::sync::broadcast::Sender;

#[async_trait]
pub trait Receiver:Sized {
    async fn bind(addr: &String, sender: Sender<String>) -> io::Result<Self>;
    async fn run(&mut self) -> Result<(), io::Error>;
    fn addr(&self) -> io::Result<std::net::SocketAddr>;
}

pub async fn start<T: Receiver>(mut receiver:T){
    println!(
        "Reciever listening; addr={}",
        receiver.addr().expect("unable to get local_addr")
    );
    if let Err(e) = receiver.run().await {
        println!("error running receiver; error={}", e);
    }
}
