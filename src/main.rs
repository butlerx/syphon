#[macro_use]
extern crate clap;
#[macro_use]
extern crate named_tuple;

use std::{env, error::Error};
use tokio::{
    self,
    sync::broadcast::{
        channel,
        Sender,
        Receiver
    }
};

mod receiver;
mod config;
mod parser;
mod uploader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(syphon =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg CONFIG: -c --config +takes_value "Config file to load (default: configs/config.toml)")
        (@arg print: -p --print ... "Print default config")
    ).get_matches();

    if matches.is_present("print") {
        return Ok(config::print_default());
    }

    let conf = config::load_config(
        matches
            .value_of("config")
            .unwrap_or("configs/config.toml")
            .to_string()
    )?;
    let (send, recv): (Sender<parser::Metric>, Receiver<parser::Metric>) = channel(1024);

    let _ = tokio::join!(
        tokio::spawn(start_udp(conf.clone(), send.clone())),
        tokio::spawn(start_tcp(conf.clone(), send.clone())),
        //tokio::spawn(start_prometheus(conf.clone(), send.clone())),
        tokio::spawn(uploader::spawn(conf.clone(), recv)),
    );

    Ok(())
}

async fn start_tcp(conf: config::Schema, sender: Sender<parser::Metric>){
    if !conf.tcp.enabled { return; }
    let server: receiver::Tcp = receiver::Receiver::bind(
        &conf.tcp.listen, sender.clone()
    ).await.expect("unable to bind to tcp port");
    receiver::start(server).await
}

async fn start_udp(conf: config::Schema, sender: Sender<parser::Metric>){
    if !conf.udp.enabled { return; }
    let server: receiver::Udp = receiver::Receiver::bind(
        &conf.udp.listen, sender.clone()
    ).await.expect("unable to bind to tcp port");
    receiver::start(server).await
}

//async fn start_prometheus(conf: config::Schema, sender: Sender<String>){
    //if !conf.prometheus.enabled { return; }
    //let server: receiver::Prometheus = receiver::Receiver::bind(
        //&conf.prometheus.listen, sender.clone()
    //).await.expect("unable to bind to Prometheus http on port");
    //receiver::start(server).await
//}
