use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the target directory where the build output is stored
    let target_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_target = env::var("PROFILE").unwrap();

    // Define the source (config.json inside src) and destination paths (target/debug or target/release)
    let src = Path::new(&target_dir).join("assets").join("config.json");
    let dest = Path::new(&target_dir)
        .join("target")
        .join(build_target)
        .join("save")
        .join("config.json");

    // Create the target directory if it doesn't exist
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Copy the config.json file to the build output folder
    fs::copy(src, dest).unwrap();
}
