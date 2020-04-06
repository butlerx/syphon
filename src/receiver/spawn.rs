use futures::future::join_all;
use tokio::{
    self,
    sync::broadcast::Sender,
};
use crate::{config, parser::Metric};
use super::{
    udp,
    tcp,
    file,
    base as receiver,
};

pub async fn spawn(conf: config::Schema, sender: Sender<Metric>) {
    let mut tasks = Vec::new();
    if conf.udp.enabled {
        let tx = sender.clone();
        let addr = conf.udp.listen.clone();
        tasks.push(tokio::spawn(async move {
            let server: udp::Server = receiver::Receiver::bind(
                &addr, tx,
            ).await.expect("unable to bind to tcp port");
            receiver::start(server).await
        }));
    }
    if conf.tcp.enabled {
        let tx = sender.clone();
        let addr = conf.tcp.listen.clone();
        tasks.push(tokio::spawn(async move {
            let server: tcp::Server = receiver::Receiver::bind(
                &addr, tx,
            ).await.expect("unable to bind to tcp port");
            receiver::start(server).await
        }));
    }
    if conf.file.enabled {
        let tx = sender.clone();
        let addr = conf.file.path.clone();
        tasks.push(tokio::spawn(async move {
            let server: file::Server = receiver::Receiver::bind(
                &addr, tx,
            ).await.expect("unable to load file");
            receiver::start(server).await
        }));
    }
    if conf.prometheus.enabled {
        // let tx = sender.clone();
        // let addr = conf.prometheus.listen.clone();
        // tokio::spawn(async move {
        //      let server: prometheus::Server = receiver::Receiver::bind(
        //         &addr, tx,
        //      ).await.expect("unable to bind to Prometheus http on port");
        //      receiver::start(server).await
        // });
    }
    join_all(tasks).await;
}

