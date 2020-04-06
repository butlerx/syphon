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
    let (send, _recv): (Sender<parser::Metric>, Receiver<parser::Metric>) = channel(1024);

    let _ = tokio::join!(
        tokio::spawn(uploader::spawn(conf.clone(), send.clone())),
        tokio::spawn(receiver::spawn(conf.clone(), send.clone())),
    );

    Ok(())
}

