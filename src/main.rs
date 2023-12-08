// Platform agnostic utility to flash Avnet RZBoard V2L

extern crate pretty_env_logger;
use clap::Parser;
use std::path::PathBuf;

mod adb_utils;

struct FlashUtil {
    cwd: String,
}

use log::info;

// Generate platform agnostic default value for serial port path
fn get_default_serial_port() -> PathBuf {
    let mut path = PathBuf::from("/");
    path.push("dev");
    path.push("ttyUSB0");
    path
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct EtchOptions {
    #[arg(long)]
    bootloader: bool,
    #[arg(long)]
    rootfs: bool,
    #[arg(long)]
    full: bool,

    #[arg(
        long,
        default_value = get_default_serial_port().into_os_string(),
        help = "Serial port used to talk to board"
    )]
    serial_port: PathBuf,
    #[arg(long, default_value_t = 115200)]
    baud_rate: u32,

    #[arg(
        long,
        help = "Set environment variable `RUST_LOG` to {trace|debug|info|warn|error} for logging"
    )]
    debug: bool,
}

fn main() {
    pretty_env_logger::init();
    let options = EtchOptions::parse();

    let flash_util = FlashUtil {
        cwd: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };

    info!("CLI options: {:?}", options);
}
