extern crate toml;
use std::fs;
use toml::de::Error;

mod schema;

pub use schema::Config as Schema;

pub fn load_config(filename: String) -> Result<schema::Config, Error> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    toml::from_str(&*contents)
}

pub fn print_default() {
    let conf: schema::Config = toml::from_str("").expect("failed to set default");
    let toml = toml::to_string_pretty(&conf).unwrap();
    println!("{}", toml)
}
