fn main() {
    println!("cargo:rerun-if-changed=build.rs"); // Always include this!
    
    // Example info output
    println!("cargo:warning=Running build script...");

    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to get git commit");

    let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Print to console (cargo will show this)
    println!("cargo:warning=Git commit hash: {}", git_hash);

    // Set environment variables for the actual code
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=NIGHTLY={}", std::env::var("NIGHTLY_BUILD").is_ok());
}