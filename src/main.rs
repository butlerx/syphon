#[macro_use]
extern crate clap;

use std::error::Error;
use std::{env};
use tokio;
use tokio::sync::broadcast::{Receiver,channel, Sender};

mod receiver;
mod config;

async fn echo(mut recv: Receiver<String>){
    loop {
        println!("message recieved {}", recv.recv().await.unwrap())
    }
}

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
    let (send, recv) = channel(1024);

    let _ = tokio::join!(
        tokio::spawn(start_udp(conf.clone(), send.clone())),
        tokio::spawn(start_tcp(conf.clone(), send.clone())),
        echo(recv)
    );

    Ok(())
}

async fn start_tcp(conf: config::Schema, sender: Sender<String>){
    if !conf.tcp.enabled { return; }
    let mut tcp_server = receiver::tcp::Server::bind(&conf.tcp.listen, sender.clone()).await.expect("unable to bind to tcp port");
    println!("Listening to tcp on: {}", tcp_server.addr().expect("unable to get local_addr"));
    tcp_server.run().await.expect("error running tcp_server");

}

async fn start_udp(conf: config::Schema, sender: Sender<String>){
    if !conf.udp.enabled { return; }
    let mut udp_server = receiver::udp::Server::bind(&conf.udp.listen, sender.clone())
        .await
        .expect("unable to bind to udp port");
    println!(
        "Listening to udp on: {}",
        udp_server.addr().expect("unable to get local_addr")
    );
    udp_server.run().await.expect("error running udp_server");
}
