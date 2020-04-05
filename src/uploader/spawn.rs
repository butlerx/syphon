use std::error::Error;
use tokio::{
    self,
    sync::broadcast::Receiver,
};
use crate::{config, parser::Metric};

pub async fn spawn(conf: config::Schema, rx: Receiver<Metric>) -> Result<(), Box<dyn Error>> {
    //for uploader in conf.uploader.udp { }
    Ok(())
}
