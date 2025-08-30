use clap::Parser; // Import the Parser trait from clap
use std::collections::HashMap; // Import hash map
use std::fs; // Import file system
mod content;
mod ascii; // Import the ascii module (ascii.rs)
mod coloring; // Import the coloring module (coloring.rs)
mod version; // Import the version module (version.rs)
mod changelog; // Import the changelog module (changelog.rs)

#[derive(Parser)]
#[command(name = "wretch", version = version::build_version(), about = "the tool to get information about your system", ignore_errors(true))]
pub struct Args {
    /// Override the ASCII art (alias: override-ascii)
    #[arg(long = "ascii", alias = "override-ascii", alias = "override")]
    ascii: Option<String>,

    /// Override username (alias: override-name)
    #[arg(long = "name", alias = "override-name")]
    username: Option<String>,

    /// Show the changelog
    #[arg(long = "changelog", short = 'c', default_value_t = false)]
    changelog: bool,
}

/// Function to get OS name for distros.
/// For example Manjaro is based on Arch Linux
pub fn get_os_name() -> (Option<String>, Option<String>) {
    let contents = match fs::read_to_string("/etc/os-release") {
        Ok(c) => c,
        Err(_) => return (None, None),
    };

    let mut map = HashMap::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, val)) = line.split_once('=') {
            let val = val.trim_matches('"');
	    let val = val.replace("\n","");
            map.insert(key.to_lowercase(), val.to_lowercase());
        }
    }

    let id = map.get("id").cloned();
    let id_like = map.get("id_like").cloned();

    (id, id_like)
}

pub fn is_generic(os_ascii_name: &str) -> bool { // Function to check if the OS is generic
    !(os_ascii_name.contains("arch") || os_ascii_name.contains("fedora") ||
       os_ascii_name.contains("windows") || os_ascii_name.contains("nixos") ||
       os_ascii_name.contains("mac") || os_ascii_name.contains("debian") ||
       os_ascii_name.contains("void") || os_ascii_name.contains("suse") ||
       os_ascii_name.contains("ubuntu") || os_ascii_name.contains("zorin"))
        
}

  

fn main() -> std::io::Result<()> {
    let args = Args::parse(); // Parse the command line arguments

    if args.changelog {
        return changelog::changelog();
    }

    print_content(&args);
    
    Ok(())
}

fn print_content(args: &Args){
    let info = content::content(args);
    let os_ascii = ascii::art(&ascii::name()); // Calls the ascii_art function to get the ASCII art based on the OS
    for (index, value) in os_ascii.iter().enumerate(){
        println!("{}{}", value, info[index]);
    }
}