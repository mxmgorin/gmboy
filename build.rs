use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let target_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_target = env::var("PROFILE").unwrap();

    let src = Path::new(&target_dir).join("assets").join("config.json");
    let dest = Path::new(&target_dir)
        .join("target")
        .join(build_target)
        .join("save")
        .join("config.json");

    // Tell Cargo to rerun this script if config.json changes
    println!("cargo:rerun-if-changed={}", src.display());

    if should_copy(&src, &dest) {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        
        fs::copy(&src, &dest).unwrap();
        println!("Copied config.json to target directory.");
    }
}

/// Determines whether `config.json` should be copied
fn should_copy(src: &Path, dest: &Path) -> bool {
    let src_metadata = fs::metadata(src).ok();
    let dest_metadata = fs::metadata(dest).ok();

    // If source file doesn't exist, do nothing
    if src_metadata.is_none() {
        return false;
    }

    // If destination file doesn't exist, copy it
    if dest_metadata.is_none() {
        return true;
    }

    let src_modified = src_metadata.unwrap().modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let dest_modified = dest_metadata.unwrap().modified().unwrap_or(SystemTime::UNIX_EPOCH);

    // Copy only if the source file is newer than the destination
    src_modified > dest_modified
}
