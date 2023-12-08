use clap::{Parser, ValueEnum};
use std::path::PathBuf;

// Generate platform agnostic default value for serial port path
pub(crate) fn get_default_serial_port() -> PathBuf {
    let mut path = PathBuf::from("/");
    path.push("dev");
    path.push("ttyUSB0");
    path
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct EtchOptions {
    #[arg(long)]
    pub(crate) bootloader: bool,
    #[arg(long)]
    pub(crate) rootfs: bool,
    #[arg(long)]
    pub(crate) full: bool,

    #[arg(
        long,
        default_value = get_default_serial_port().into_os_string(),
        help = "Serial port used to talk to board"
    )]
    pub(crate) serial_port: PathBuf,
    #[arg(long, default_value_t = 115200)]
    pub(crate) baud_rate: u32,

    #[arg(
        long,
        default_value = "",
        help = "IP address assigned to board during flashing."
    )]
    pub(crate) static_ip: String,

    #[arg(long, value_enum, default_value_t = FlashTarget::EMMC)]
    pub(crate) flash_target: FlashTarget,

    #[arg(
        long,
        help = "Add debug logging. Notice: you can set environment variable `RUST_LOG` to {trace|debug|info|warn|error} for logging"
    )]
    pub(crate) debug: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum FlashTarget {
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
