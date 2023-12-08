extern crate pretty_env_logger;
use clap::Parser;
use std::env;

mod adb_utils;
mod cli;

struct FlashUtil {
    cwd: String,
}

use log::info;

fn main() {
    // pretty_env_logger::init();
    let options = cli::EtchOptions::parse();

    if options.debug {
        env::set_var("RUST_LOG", "debug");
    }

    pretty_env_logger::init_custom_env("RUST_LOG=debug");
    let flash_util = FlashUtil {
        cwd: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };

    info!("CLI options: {:?}", options);
}
