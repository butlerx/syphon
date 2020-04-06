use std::{
    error::Error,
    path::Path,
};
use tokio::{
    prelude::*,
    fs::OpenOptions,
    sync::broadcast::Receiver,
};
use regex::Regex;
use crate::parser::Metric;
use super::graphite;

pub async fn uploader(
    path: String,
    pattern: String,
    mut rx: Receiver<Metric>
) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(&path);
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(&file_path)
        .await?;
    info!(
        "conected to remote endpoint; proto={} remote_addr={} pattern={}",
        "file",
        file_path.display(),
        pattern
    );
    let re = Regex::new(&pattern).unwrap();
    loop {
        let res = rx.recv().await.unwrap() ;
        if re.is_match(res.path()) {
            file.write_all(&graphite::format(res).into_bytes()).await?;
            debug!(
                "message sent; proto={} remote_addr={} pattern={}",
                "file",
                file_path.display(),
                pattern
            );
        }
    }
}
