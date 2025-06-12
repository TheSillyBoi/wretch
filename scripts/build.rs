use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;
use std::env;
use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct Page {
    version: String,
    body: String,
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=CHANGELOG.md");
    println!("cargo:rerun-if-changed={}/changelog_data.json", out_dir);
    
    
    // Example info output
    println!("cargo:warning=Running build script...");

    // Get git hash
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to get git commit");

    let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Print to console
    println!("cargo:warning=Git commit hash: {}", git_hash);
    println!("cargo:warning=Out Dir: {}", out_dir);

    // Parse and generate changelog JSON
    println!("cargo:warning=Parsing changelog...");
    if let Err(e) = generate_changelog_json() {
        println!("cargo:warning=Error generating changelog JSON: {}", e);
    } else {
        println!("cargo:warning=Changelog JSON generated successfully");
    }

    // Set environment variables for the actual code
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=NIGHTLY={}", std::env::var("NIGHTLY_BUILD").is_ok());
    println!("cargo:rustc-env=CHANGELOGPATH={}/changelog_data.json", out_dir);
}

fn format_bullet_points(line: &str) -> String {
    // Regex to match lines starting with whitespace (if any) followed by a bullet point
    let re = Regex::new(r"^(\s*)([-*])\s+").unwrap();
    
    // Replace bullet points with •
    if let Some(caps) = re.captures(line) {
        let whitespace = &caps[1]; // Capture the whitespace
        return format!("{}• {}", whitespace, &line[caps[0].len()..]);
    }
    
    line.to_string()
}

fn generate_changelog_json() -> Result<(), Box<dyn std::error::Error>> {
    // Define paths
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let changelog_path = manifest_dir.join("CHANGELOG.md");
    let output_path = out_dir.join("changelog_data.json");
    
    // Read the changelog
    let mut content = String::new();
    File::open(&changelog_path)?.read_to_string(&mut content)?;
    
    // Parse the changelog
    let re_version = Regex::new(r"## (v\d+\.\d+\.\d+)")?;
    let mut pages: Vec<Page> = Vec::new();
    
    // Split the changelog by version headers
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut version = String::new();
    let mut body = Vec::new();
    
    for line in content.lines() {
        if let Some(caps) = re_version.captures(line) {
            // If we have a previous version, save it
            if !version.is_empty() {
                sections.push((version, body.join("\n")));
                body.clear();
            }
            version = caps[1].to_string();
        } else if !version.is_empty() {
            // Format bullet points before adding to body
            body.push(format_bullet_points(line));
        }
    }
    
    // Don't forget the last version
    if !version.is_empty() {
        sections.push((version, body.join("\n")));
    }
    
    // Convert sections to Page objects
    for (version, body) in sections {
        // Trim the body and ensure it's not empty
        let trimmed_body = body.trim();
        if !trimmed_body.is_empty() {
            pages.push(Page {
                version,
                body: trimmed_body.to_string(),
            });
        }
    }
    
    // Generate new JSON content
    let new_json = serde_json::to_string_pretty(&pages)?;
    
    // Check if file exists and has the same content
    let should_write = if output_path.exists() {
        let mut existing_content = String::new();
        match File::open(&output_path) {
            Ok(mut file) => {
                file.read_to_string(&mut existing_content)?;
                existing_content != new_json
            },
            Err(_) => true // If we can't open the file, write anyway
        }
    } else {
        true // File doesn't exist, must write
    };
    
    // Only write if content is different
    if should_write {
        println!("cargo:warning=Updating changelog_data.json");
        let mut file = File::create(&output_path)?;
        file.write_all(new_json.as_bytes())?;
    } else {
        println!("cargo:warning=Changelog content unchanged, skipping write");
    }
    
    Ok(())
}