#[macro_use] extern crate error_chain;
extern crate file;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate rmp_serde;
extern crate structopt;
#[macro_use] extern crate structopt_derive;

mod error;

use std::process;
use error::errors::*;
use rmp_serde::{Deserializer, Serializer};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Authentication Management", about = "Program to perform authentication management.")]
struct MainConfig {
    #[structopt(long = "source", help = "Source estimate matrix msgpack path")]
    source_mat_path: String,

    #[structopt(long = "index", help = "j-selected index msgpack path")]
    selected_index_path: String,
}

fn run() -> Result<()> {
    let config = MainConfig::from_args();
    
    let selected_index_buf = file::get(&config.selected_index_path).chain_err(|| "Unable to get selected index msgpack")?;
    let selected_index: u32 = rmp_serde::from_slice(&selected_index_buf[..]).chain_err(|| "Unable to deserialize into selected index")?;

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {
            info!("Program completed!");
            process::exit(0)
        },

        Err(ref e) => {
            error!("Error: {}", e);

            for e in e.iter().skip(1) {
                error!("> Caused by: {}", e);
            }

            process::exit(1);
        },
    }
}