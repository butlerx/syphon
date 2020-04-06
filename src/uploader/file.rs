use std::{
    error::Error,
    path::Path,
};
use tokio::{
    prelude::*,
    fs::OpenOptions,
    sync::broadcast::Receiver,
};
use crate::parser::Metric;
use super::graphite;

pub async fn uploader(path: String, mut rx: Receiver<Metric>) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(&path);
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(&file_path)
        .await?;
    println!("Writing to file; file={}", file_path.display());

    loop {
        let res = rx.recv().await.unwrap() ;
        file.write_all(&graphite::format(res).into_bytes()).await?;
    }
}
