#[macro_use]
extern crate clap;
#[macro_use]
extern crate named_tuple;
use kv_log_macro as log;

use std::{env, error::Error};
use tokio::{
    self,
    sync::broadcast::{channel, Receiver, Sender},
};

mod config;
mod parser;
mod receiver;
mod uploader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(syphon =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg CONFIG: -c --config +takes_value "Config file to load (default: configs/config.toml)")
        (@arg print: -p --print "Print default config")
        (@arg verbosity: -v ... "Increase message verbosity")
        (@arg quiet: -q "Silence all output")
    )
    .get_matches();

    if matches.is_present("print") {
        config::print_default();
        return Ok(());
    }

    json_env_logger::init();
    json_env_logger::panic_hook();

    let conf_path = matches.value_of("config").unwrap_or("configs/config.toml");
    log::debug!("loading config", { path: conf_path });
    let conf = config::load_config(conf_path.to_string())?;
    log::info!("config loaded", { path: conf_path });

    let (send, _recv): (Sender<parser::Metric>, Receiver<parser::Metric>) = channel(1024);

    let _ = tokio::join!(
        tokio::spawn(uploader::spawn(conf.clone(), send.clone())),
        tokio::spawn(receiver::spawn(conf.clone(), send.clone())),
    );
    log::info!("shutting down server");
    Ok(())
}
