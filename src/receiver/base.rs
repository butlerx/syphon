use std::{io};
use async_trait::async_trait;

#[async_trait]
pub trait Receiver:Sized {
    async fn bind(addr: &String) -> io::Result<Self>;
    async fn run(&mut self) -> Result<(), io::Error>;
    fn addr(&self) -> io::Result<std::net::SocketAddr>;
}
