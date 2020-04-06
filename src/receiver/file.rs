use async_trait::async_trait;
use crate::parser::{graphite, Metric};
use std::path::Path;
use super::base;
use tokio::{
    self,
    io::{
       self,
       BufReader,
       AsyncBufReadExt
    },
    fs::File,
    sync::broadcast::Sender,
};

pub struct Server {
    sender: Sender<Metric>,
    path: String,
}

#[async_trait]
impl base::Receiver for Server {
    async fn bind(addr: &String, sender: Sender<Metric>) -> io::Result<Server> {
        let file_path = Path::new(addr);
        Ok(Server{ sender, path: file_path.display().to_string() })
    }

    fn addr(&self) -> io::Result<String> {
        let path = &self.path.clone();
        Ok(path.to_string())
    }

    async fn run(&mut self) -> Result<(), io::Error> {
        let file = File::open(&self.path).await?;
        let mut reader = BufReader::new(file);
        let sender = self.sender.clone();
        tokio::spawn(async move {
            loop {
                let mut buffer = String::new();
                reader.read_line(&mut buffer).await.expect("unable to read line");
                for metric in graphite::parse(buffer) {
                    sender
                        .send(metric)
                        .expect("failed to write data to channel");
                }
            }
        });
        Ok(())
    }
}
