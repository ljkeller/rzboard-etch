extern crate pretty_env_logger;
use clap::{Parser, ValueEnum};
use std::env;
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
        default_value = "",
        help = "IP address assigned to board during flashing."
    )]
    static_ip: String,

    #[arg(long, value_enum, default_value_t = FlashTarget::EMMC)]
    flash_target: FlashTarget,

    #[arg(
        long,
        help = "Add debug logging. Notice: you can set environment variable `RUST_LOG` to {trace|debug|info|warn|error} for logging"
    )]
    debug: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FlashTarget {
    EMMC,
    QSPI,
}

impl std::fmt::Debug for FlashTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlashTarget::EMMC => write!(f, "EMMC"),
            FlashTarget::QSPI => write!(f, "QSPI"),
        }
    }
}

impl FlashTarget {}

fn main() {
    // pretty_env_logger::init();
    let options = EtchOptions::parse();

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
