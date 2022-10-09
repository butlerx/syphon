use super::graphite;
use crate::parser::Metric;
use kv_log_macro as log;
use regex::Regex;
use std::{error::Error, path::Path};
use tokio::{fs::OpenOptions, prelude::*, sync::broadcast::Receiver};

pub async fn uploader(
    path: String,
    pattern: String,
    mut rx: Receiver<Metric>,
) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(&path);
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(&file_path)
        .await?;
    log::info!( "conected to remote endpoint", { proto: "file", remote_addr: path, patter: pattern });
    let re = Regex::new(&pattern).unwrap();
    loop {
        let res = rx.recv().await.unwrap();
        if re.is_match(res.path()) {
            file.write_all(&graphite::format(res).into_bytes()).await?;
            log::debug!( "message sent", { proto: "file", remote_addr: path, patter: pattern });
        }
    }
}
