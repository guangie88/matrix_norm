#[macro_use] extern crate error_chain;
extern crate file;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate rmp_serde;
extern crate simple_logger;
#[macro_use] extern crate serde_derive;
extern crate structopt;
#[macro_use] extern crate structopt_derive;

mod error;

use std::process;
use error::errors::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Authentication Management", about = "Program to perform authentication management.")]
struct MainConfig {
    #[structopt(short = "s", long = "source", help = "Source estimate matrix msgpack path")]
    source_mat_path: String,

    #[structopt(short = "i", long = "index", help = "j-selected index msgpack path")]
    selected_index_path: String,

    #[structopt(long = "log", help = "Log configuration path")]
    log_config_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SourceMat {
    rows: u32,
    cols: u32,
}

fn run() -> Result<()> {
    let config = MainConfig::from_args();

    if let &Some(ref log_config_path) = &config.log_config_path {
        log4rs::init_file(log_config_path, Default::default())
            .chain_err(|| format!("Unable to initialize log4rs logger with the given config file at '{}'", log_config_path))?;
    } else {
        simple_logger::init()
            .chain_err(|| "Unable to initialize default logger")?;
    }
    
    let selected_index_buf = file::get(&config.selected_index_path)
        .chain_err(|| "Unable to read selected index msgpack")?;

    let selected_index: u32 = rmp_serde::from_slice(&selected_index_buf[..])
        .chain_err(|| "Unable to deserialize into selected index")?;

    let source_mat_buf = file::get(&config.source_mat_path)
        .chain_err(|| "Unable to read source estimate matrix msgpack")?;

    let source_mat: SourceMat = rmp_serde::from_slice(&source_mat_buf[..])
        .chain_err(|| "Unable to deserialize into source estimate matrix")?;

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
