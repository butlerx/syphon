use futures::future::join_all;
use tokio::{
    self,
    sync::broadcast::Sender,
};
use crate::{config, parser::Metric};
use super::{udp, tcp, file};

pub async fn spawn(conf: config::Schema, sender: Sender<Metric>) {
    let mut tasks = Vec::new();
    for uploader in conf.uploader.udp {
        if uploader.enabled {
            let tx = sender.clone();
            tasks.push(tokio::spawn(async move {
                udp::uploader(
                    uploader.host.clone(),
                    uploader.port.clone(),
                    tx.subscribe()
                ).await.expect("failed to send to udp")
            }));
        }
    }
    for uploader in conf.uploader.tcp {
        if uploader.enabled {
            let tx = sender.clone();
            tasks.push(tokio::spawn(async move {
                tcp::uploader(
                    uploader.host.clone(),
                    uploader.port.clone(),
                    tx.subscribe()
                ).await.expect("failed to send to tcp")
            }));
        }
    }
    for uploader in conf.uploader.grpc {
        if uploader.enabled {
            let _tx = sender.clone();
            tasks.push(tokio::spawn(async move {
                //grpc::uploader(
                //    uploader.host.clone(),
                //    uploader.port.clone(),
                //    tx.subscribe()
                //)
            }));
        }
    }
    for uploader in conf.uploader.file {
        if uploader.enabled {
            let tx = sender.clone();
            tasks.push(tokio::spawn(async move {
                file::uploader(
                    uploader.path.clone(),
                    tx.subscribe()
                ).await.expect("failed to write to file")
            }));
        }
    }
    join_all(tasks).await;
}
