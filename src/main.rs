// Platform agnostic utility to flash Avnet RZBoard V2L

use core::panic;

struct FlashUtil {
    cwd: String,
}

struct AdbExtractor<'a> {
    cwd: &'a str,
}

impl<'a> AdbExtractor<'a> {
    fn extract(&self) {
        println!("Extracting ADB");
        // TODO! Ensure this is equivalent to sys.platform
        let operating_sys = std::env::consts::OS;

        let platform_tools = format!("{}/adb/platform-tools", self.cwd);
        let already_extracted = std::path::Path::new(&platform_tools).is_dir();

        if already_extracted {
            println!("ADB already extracted");
            return;
        }

        let archive_fp: String;
        match operating_sys {
            "linux" => {
                archive_fp = format!("{}/adb/platform-tools-latest-linux.zip", self.cwd);
            }
            "macos" => {
                archive_fp = format!("{}/adb/platform-tools-latest-darwin.zip", self.cwd);
            }
            "windows" => {
                archive_fp = format!("{}/adb/platform-tools-latest-windows.zip", self.cwd);
            }
            _ => {
                panic!("Unsupported platform: {}", operating_sys);
            }
        }
        let archive_path = std::path::Path::new(&archive_fp);

        // TODO: Check if we can use Rust's ADB client instead of maintaining the zips
        let archive_file = match std::fs::File::open(archive_path) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file: {:?}", e),
        };

        // let mut zip_archive =
        // zip::read::ZipArchive::new(archive_file).unwrap();
        let mut zip_archive = match zip::read::ZipArchive::new(&archive_file) {
            Ok(archive) => archive,
            Err(e) => panic!("Error creating zip_archive object: {:?}", e),
        };
        zip_archive
            .extract(std::path::Path::new(&format!("{}/adb", self.cwd)))
            .expect("Failure to extract ADB, cannot continue");

        if std::env::consts::OS != "windows" {
            // TODO: give permission to execute
            let fastboot_fp =
                std::fs::File::open(format!("{}/adb/platform-tools/fastboot", self.cwd))
                    .expect("Error opening /adb/platform-tools/fastboot");
            // Notice we can only use this on Unix systems!
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;
            let exe_perms = Permissions::from_mode(0o755);
            fastboot_fp.set_permissions(exe_perms).expect("Error setting permissions on fastboot. Do you have sufficient permissions to make fastboot executable?");
        }

    }
}

fn main() {
    let flash_util = FlashUtil {
        cwd: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };

    let adb_extractor = AdbExtractor {
        cwd: &flash_util.cwd,
    };
    adb_extractor.extract();
}
