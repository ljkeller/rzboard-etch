extern crate pretty_env_logger;
use clap::Parser;
use std::env;

mod adb_utils;
mod cli;

use log::info;

mod flash {
    use crate::cli::EtchOptions;
    use log::{debug, info};
    use serial2::SerialPort;
    use std::io::BufRead;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::Duration;

    pub fn read_until<R: BufRead>(
        reader: &mut R,
        end: &str,
        timeout_lines: Option<u128>,
    ) -> Result<(), std::io::Error> {
        let mut idx = 0;
        let mut line = String::new();
        let mut search = true;
        while search {
            debug!("Reading line {}", idx);

            let read_size = reader.read_line(&mut line)?;
            if read_size == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Couldn't find end of stream",
                ));
            }
            if line.contains(end) {
                info!("Read until: {:?} was found", end);
                return Ok(());
            }

            // read_line continually appends, so we should clear buffer
            line.clear();

            if timeout_lines.is_some() && idx < timeout_lines.unwrap() {
                idx += 1;
                if idx == timeout_lines.unwrap() {
                    search = false;
                    info!("Timeout reached");
                }
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "Couldn't find end of stream within line timeout",
        ))
    }

    pub struct FlashManager {
        flash_worker: Box<dyn Flasher>,
        flash_instructions: EtchOptions,
        path: PathBuf,
    }

    impl FlashManager {
        pub fn new<P: AsRef<Path>>(config: EtchOptions, p: P) -> Self {
            // TODO
            Self {
                flash_worker: Box::new(EmmcBootloaderFlasher {
                    config: config.clone(),
                }),
                flash_instructions: config,
                path: PathBuf::from(p.as_ref()),
            }
        }

        pub fn process_flash_instructions(&self) -> Result<(), std::io::Error> {
            // TODO
            info!("Handling flash instructions");
            self.flash_worker.flash()
        }
    }

    pub trait Flasher {
        fn flash(&self) -> Result<(), std::io::Error>;
    }

    struct EmmcBootloaderFlasher {
        config: EtchOptions,
    }

    impl Flasher for EmmcBootloaderFlasher {
        fn flash(&self) -> Result<(), std::io::Error> {
            // TODO
            info!("Flashing EmmcBootloaderFlasher");

            debug!("Flashing /Users/lucaskeller/code/open_source/rzboard-etch/vlp_304_images/Flash_Writer_SCIF_rzboard.mot");
            let flash_writer_data = std::fs::read("/Users/lucaskeller/code/open_source/rzboard-etch/vlp_304_images/Flash_Writer_SCIF_rzboard.mot")?;
            // println!("Flash writer data: {:?}", flash_writer_data);
            println!("Available port: {:?}", SerialPort::available_ports()?);

            let mut port = SerialPort::open(&self.config.serial_port, self.config.baud_rate)?;
            port.set_read_timeout(Duration::from_secs(15))?;

            info!("Waiting for board to be ready");

            let mut reader = std::io::BufReader::new(&port);

            read_until(&mut reader, "!\r\n", None)?;

            // RZBoard firmware doesnt send any EOS or special character...
            // Each step of firmware loading process will be custom
            // For example, SCIF mode ends in "!\r\n" stream
            // let mut buff = vec![];
            // let sz = reader.read_until(b'!', &mut buff)?;
            // info!("Read buf: {:?}", String::from_utf8_lossy(&buff[..sz]));

            // info!("Writing flash writer to board");
            // port.write_all(&flash_writer_data)?;

            // std::io::Read::read_to_string(&mut port, &mut read_buf)?;
            // info!("Read buf: {:?}", read_buf);
            Ok(())
        }
    }

    struct QspiBootloaderFlasher {
        config: EtchOptions,
    }
    impl Flasher for QspiBootloaderFlasher {
        fn flash(&self) -> Result<(), std::io::Error> {
            // TODO
            info!("Flashing QspiBootloaderFlasher");
            Ok(())
        }
    }

    struct KernelEmmcFlasher {
        config: EtchOptions,
    }
    impl Flasher for KernelEmmcFlasher {
        fn flash(&self) -> Result<(), std::io::Error> {
            // TODO
            info!("Flashing KernelEmmcFlasher");
            Ok(())
        }
    }
}

fn main() {
    let options = cli::EtchOptions::parse();
    if options.debug {
        env::set_var("RUST_LOG", "debug");
    }

    pretty_env_logger::init();

    let flash_manager = flash::FlashManager::new(options.clone(), std::env::current_dir().unwrap());
    flash_manager.process_flash_instructions().unwrap();

    info!("CLI options: {:?}", options);
}
