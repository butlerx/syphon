use crate::parser::{graphite, Metric};
use kv_log_macro as log;
use std::path::Path;
use tokio::{
    self,
    fs::File,
    io::{self, AsyncBufReadExt, BufReader},
    sync::broadcast::Sender,
};

pub async fn bind(path: String, sender: Sender<Metric>) -> Result<(), io::Error> {
    let file_path = Path::new(&path);
    let file = File::open(&file_path).await?;
    let mut reader = BufReader::new(file);
    log::info!( "Reciever listening", { proto: "file", addr: path });
    let sender = sender.clone();
    tokio::spawn(async move {
        loop {
            let mut buffer = String::new();
            reader
                .read_line(&mut buffer)
                .await
                .expect("unable to read line");
            for metric in graphite::parse(buffer) {
                sender
                    .send(metric)
                    .expect("failed to write data to channel");
            }
        }
    });
    Ok(())
}
