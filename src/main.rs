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

    #[structopt(short = "o", long = "out", help = "output sample path")]
    output_path: String,

    #[structopt(long = "log", help = "Log configuration path")]
    log_config_path: Option<String>,
}

// #[derive(Serialize, Deserialize)]
// struct Complex<T: Serialize + for<'de_inner> Deserialize<'de_inner>> {
//     real: T,
//     imag: T,
// }

#[derive(Serialize, Deserialize)]
struct Cpxf64 {
    real: f64,
    imag: f64,
}

#[derive(Serialize, Deserialize)]
struct SourceMat {
    rows: usize,
    cols: usize,
    raw_values: Vec<Cpxf64>,
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
    
    // deserialize
    let selected_index_buf = file::get(&config.selected_index_path)
        .chain_err(|| "Unable to read selected index msgpack")?;

    let selected_index: usize = rmp_serde::from_slice(&selected_index_buf[..])
        .chain_err(|| "Unable to deserialize into selected index")?;

    let source_mat_buf = file::get(&config.source_mat_path)
        .chain_err(|| "Unable to read source estimate matrix msgpack")?;

    let source_mat: SourceMat = rmp_serde::from_slice(&source_mat_buf[..])
        .chain_err(|| "Unable to deserialize into source estimate matrix")?;

    // normalize to i16 (i32 write width) and flatten
    let offset = selected_index * source_mat.cols;
    let selected_row = &source_mat.raw_values[offset..offset + source_mat.cols];

    let flattened_values: Vec<f64> = selected_row.iter()
        .flat_map(|cpx| vec![cpx.real, cpx.imag].into_iter())
        .collect();

    let max_value = flattened_values.iter()
        .max_by(|x, y| {
            x.abs()
                .partial_cmp(&y.abs())
                .expect(&format!("{} cannot be partial compared with {}", x, y))
        })
        .ok_or_else(|| "Unable to find the max value in the source matrix")?;

    let norm_values: Vec<i32> = flattened_values.iter()
        .map(|v| (v * (std::i16::MAX as f64) / max_value) as i32)
        .collect();

    file::put(&config.output_path, unsafe { Box::from_raw(Box::into_raw(norm_values.into_boxed_slice()) as *mut [u8])})
        .chain_err(|| format!("Unable to write sample into '{}'", config.output_path))?;

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
