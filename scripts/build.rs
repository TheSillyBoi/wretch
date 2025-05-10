use std::process::Command;
use std::env;

fn main() {
    // Get git commit hash
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to get git commit");
    let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Detect nightly from env var (set in CI)
    let is_nightly = env::var("NIGHTLY_BUILD").is_ok();

    // Export it to code
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=NIGHTLY={}", is_nightly);
}