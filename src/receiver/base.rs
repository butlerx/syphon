use std::io;
use async_trait::async_trait;
use tokio::sync::broadcast::Sender;
use crate::parser::Metric;

#[async_trait]
pub trait Receiver:Sized {
    async fn bind(addr: &String, sender: Sender<Metric>) -> io::Result<Self>;
    async fn run(&mut self) -> Result<(), io::Error>;
    fn addr(&self) -> io::Result<String>;
}

pub async fn start<T: Receiver>(mut receiver:T){
    info!(
        "Reciever listening; addr={}",
        receiver.addr().expect("unable to get local_addr")
    );
    if let Err(e) = receiver.run().await {
        error!("error running receiver; error={}", e);
    }
}
