// Platform agnostic utility to flash Avnet RZBoard V2L

mod adb_utils;

struct FlashUtil {
    cwd: String,
}

fn main() {
    let flash_util = FlashUtil {
        cwd: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };

    let adb_extractor = adb_utils::extract::AdbExtractor {
        cwd: &flash_util.cwd,
    };
    adb_extractor.extract();
}
