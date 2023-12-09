extern crate pretty_env_logger;
use clap::Parser;
use std::env;

mod adb_utils;
mod cli;

use log::info;

mod flash {
    use crate::cli::EtchOptions;
    use log::info;
    use std::path::Path;
    use std::path::PathBuf;

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

        pub fn process_flash_instructions(&self) {
            // TODO
            info!("Handling flash instructions");
            self.flash_worker.flash();
        }
    }

    pub trait Flasher {
        fn flash(&self);
    }

    struct EmmcBootloaderFlasher {
        config: EtchOptions,
    }

    impl Flasher for EmmcBootloaderFlasher {
        fn flash(&self) {
            // TODO
            info!("Flashing EmmcBootloaderFlasher");
        }
    }

    struct QspiBootloaderFlasher {
        config: EtchOptions,
    }
    impl Flasher for QspiBootloaderFlasher {
        fn flash(&self) {
            // TODO
            info!("Flashing QspiBootloaderFlasher");
        }
    }

    struct KernelEmmcFlasher {
        config: EtchOptions,
    }
    impl Flasher for KernelEmmcFlasher {
        fn flash(&self) {
            // TODO
            info!("Flashing KernelEmmcFlasher");
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

    info!("CLI options: {:?}", options);
}
