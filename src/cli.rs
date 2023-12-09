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
#[command(author = "Lucas K. <lucas.keller@avnet.com>")]
#[command(about = "Flashing utility for Avnet RZBoard V2L")]
pub(crate) struct EtchOptions {
    // core
    #[arg(long)]
    pub(crate) bootloader: bool,
    #[arg(long)]
    pub(crate) rootfs: bool,
    #[arg(long)]
    pub(crate) full: bool,
    // core end

    // Image conf
    #[arg(
        long,
        help = "\nAbsolute path to images dir (used only with --bootloader, --rootfs, or --full to overwrite <SCRIPT_DIR>)"
    )]
    pub(crate) image_path: Option<PathBuf>,
    #[arg(long, help = "Path to Flash Writer image")]
    pub(crate) flash_writer: Option<PathBuf>,
    #[arg(long, help = "Path to Bl2 image")]
    pub(crate) image_bl2: Option<PathBuf>,
    #[arg(long, help = "Path to FIP image")]
    pub(crate) image_fip: Option<PathBuf>,
    #[arg(long, help = "Path to rootfs image")]
    pub(crate) image_rootfs: Option<PathBuf>,
    // Image conf end

    // Serial conf
    #[arg(
        long,
        default_value = get_default_serial_port().into_os_string(),
        help = "\nSerial port used to talk to board"
    )]
    pub(crate) serial_port: PathBuf,
    #[arg(long, default_value_t = 115200)]
    pub(crate) baud_rate: u32,
    // Serial conf end

    // Network conf
    #[arg(
        long,
        default_value = "",
        help = "\nIP address assigned to board during flashing.\n"
    )]
    pub(crate) static_ip: String,
    // Network conf end

    // Flash conf
    #[arg(long, value_enum, default_value_t = FlashTarget::EMMC)]
    pub(crate) flash_target: FlashTarget,
    // Flash conf end

    // Debug
    #[arg(
        long,
        help = "\nAdd debug logging. Notice: you can set environment variable `RUST_LOG` to {trace|debug|info|warn|error} for logging"
    )]
    pub(crate) debug: bool,
    // Debug end
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
