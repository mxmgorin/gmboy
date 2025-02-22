fn main() {
    println!("cargo:rerun-if-changed=assets/config.json");
    println!("Running build.rs...");
}