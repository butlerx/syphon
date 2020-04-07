use crate::parser::{graphite, Metric};
use std::path::Path;
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

pub async fn bind(path: String, sender: Sender<Metric>) -> Result<(), io::Error> {
    let file_path = Path::new(&path);
    let file = File::open(&file_path).await?;
    let mut reader = BufReader::new(file);
    info!(
        "Reciever listening; proto={} path={}",
        "file",
        file_path.display()
    );
    let sender = sender.clone();
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
