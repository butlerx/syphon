use super::{
    file,
    //prometheus,
    tcp,
    udp,
};
use crate::{config, parser::Metric};
use futures::future::join_all;
use tokio::{self, sync::broadcast::Sender};

pub async fn spawn(conf: config::Schema, sender: Sender<Metric>) {
    let mut tasks = Vec::new();
    if conf.udp.enabled {
        tasks.push(tokio::spawn(udp::bind(
            conf.udp.listen.clone(),
            sender.clone(),
        )));
    }
    if conf.tcp.enabled {
        tasks.push(tokio::spawn(tcp::bind(
            conf.tcp.listen.clone(),
            sender.clone(),
        )));
    }
    if conf.file.enabled {
        tasks.push(tokio::spawn(file::bind(
            conf.file.path.clone(),
            sender.clone(),
        )));
    }
    // if conf.prometheus.enabled {
    //     tasks.push(tokio::spawn(prometheus::bind(
    //         conf.prometheus.listen.clone(),
    //         sender.clone()
    //     )));
    // }
    join_all(tasks).await;
}
