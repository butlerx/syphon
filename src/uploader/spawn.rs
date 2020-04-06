use tokio::{
    self,
    sync::broadcast::Sender,
};
use crate::{config, parser::Metric};
use super::{udp, tcp};

pub async fn spawn(conf: config::Schema, sender: Sender<Metric>) {
    for uploader in conf.uploader.udp {
        if uploader.enabled {
            let tx = sender.clone();
            tokio::spawn(async move {
                udp::uploader(
                    uploader.host.clone(),
                    uploader.port.clone(),
                    tx.subscribe()
                )
            });
        }
    }
    for uploader in conf.uploader.tcp {
        if uploader.enabled {
            let tx = sender.clone();
            tokio::spawn(async move {
                tcp::uploader(
                    uploader.host.clone(),
                    uploader.port.clone(),
                    tx.subscribe()
                )
            });
        }
    }
}
