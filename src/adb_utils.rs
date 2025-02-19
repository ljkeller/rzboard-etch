use core::panic;
use log::{debug, info};
use std::fs::File;
use std::path::Path;

pub struct AdbExtractor<'a> {
    pub cwd: &'a str,
}

impl<'a> AdbExtractor<'a> {
    pub fn extract(&self) {
        // TODO! Ensure this is equivalent to sys.platform
        let operating_sys = std::env::consts::OS;

        let platform_tools_str = format!("{}/adb/platform-tools", self.cwd);
        let already_extracted = Path::new(&platform_tools_str).is_dir();
        if already_extracted {
            info!("ADB already extracted");
            return;
        }

        let platform_specific_archive_str: String;
        match operating_sys {
            "linux" => {
                platform_specific_archive_str =
                    format!("{}/adb/platform-tools-latest-linux.zip", self.cwd);
            }
            "macos" => {
                platform_specific_archive_str =
                    format!("{}/adb/platform-tools-latest-darwin.zip", self.cwd);
            }
            "windows" => {
                platform_specific_archive_str =
                    format!("{}/adb/platform-tools-latest-windows.zip", self.cwd);
            }
            _ => {
                panic!("Unsupported platform: {}", operating_sys);
            }
        }
        info!(
            "Platform-specific archive: {}",
            platform_specific_archive_str
        );

        // TODO: Check if we can use Rust's ADB client instead of maintaining the zips
        let archive_file = match File::open(&platform_specific_archive_str) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file: {:?}", e),
        };

        let mut zip_archive = match zip::read::ZipArchive::new(&archive_file) {
            Ok(archive) => archive,
            Err(e) => panic!("Error creating zip_archive object: {:?}", e),
        };
        zip_archive
            .extract(Path::new(&format!("{}/adb", self.cwd)))
            .expect("Failure to extract ADB, cannot continue");
        println!("Successfuly extracted {}", platform_specific_archive_str);

        if std::env::consts::OS != "windows" {
            debug!("Setting fastboot as executable for non-windows platform");

            let fastboot_fp = File::open(format!("{}/adb/platform-tools/fastboot", self.cwd))
                .expect("Error opening /adb/platform-tools/fastboot");
            // Notice we can only use this on Unix systems!
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;
            let exe_perms = Permissions::from_mode(0o755);
            fastboot_fp.set_permissions(exe_perms).expect("Error setting permissions on fastboot. Do you have sufficient permissions to make fastboot executable?");
        }
    }
}
