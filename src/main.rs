// Platform agnostic utility to flash Avnet RZBoard V2L

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
        let already_extracted = std::path::Path::exists(std::path::Path::new(&platform_tools));

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

        if !std::path::Path::is_file(std::path::Path::new(&archive_fp)) {
            panic!("Platform specific ADB archive not found: {}.", archive_fp);
        }

        // TODO: Extract archive
        // TODO: give permission to execute on non-windows systems

        println!("Extracting ADB from {}", archive_fp);
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
