use clap::Parser; // Import the Parser trait from clap
use colored::Colorize; // Import the Colorize trait from colored
use sysinfo::System; // Import the System struct from sysinfo
use std::collections::HashMap; // Import hash map
use std::fs; // Import file system

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

    let color = coloring::info_color(); // Calls the info_color function to get the color values
    let os_ascii = ascii::art(&ascii::name()); // Calls the ascii_art function to get the ASCII art based on the OS

    let mut sys = System::new_all(); // Gather system information
    sys.refresh_all(); // Refresh all system information

    print!("{}", os_ascii[0]); // prints the first line of the ascii art
    println!(
        "{}'s System information",
        if args.username.is_some() {
            args.username
                .expect("args.username was Option::None, this should not happen please report this to us at https://github.com/thesillyboi/wretch/issues/new")
                .to_string()
        } else {
            whoami::realname()      
        }
    ); // Prints the person's name using the whoami crate
    print!("{}", os_ascii[1]); // prints the second line of the ascii art
    println!(
        "System name: {} {}",
        System::name()
            .unwrap_or_else(|| "Unknown".to_string())
            .truecolor(color[0], color[1], color[2]),
        format!("({})", System::cpu_arch()).truecolor(color[0], color[1], color[2])
    ); // Prints the system name and CPU architecture

    print!("{}", os_ascii[2]); // prints the third line of the ascii art
    println!(
        "Operating System Version: {}",
        System::long_os_version()
            .unwrap_or_default()
            .truecolor(color[0], color[1], color[2])
    ); // Prints the OS version

    print!("{}", os_ascii[3]); // prints the fourth line of the ascii art
    println!(
        "System kernel version: {}",
        System::kernel_long_version().truecolor(color[0], color[1], color[2])
    ); // Prints the kernel version

    print!("{}", os_ascii[4]); // prints the fifth line of the ascii art
    println!(
        "Hostname: {}",
        format!("{}@{}", whoami::username(), System::host_name().unwrap())
            .truecolor(color[0], color[1], color[2])
    ); // Prints the hostname and username

    let mem_used_mb = sys.used_memory() / 1024 / 1024; // Converts used to MB
    let mem_total_mb = sys.total_memory() / 1024 / 1024; // Converts Total to MB
    let mem_usage_prc = sys.used_memory() * 100 / sys.total_memory(); // Calculates the percentage of memory used
    print!("{}", os_ascii[5]); // prints the sixth line of the ascii art
    println!(
        "Memory Usage: {} {}",
        format!("{}/{} MB", mem_used_mb, mem_total_mb).truecolor(color[0], color[1], color[2]),
        format!("({}%)", mem_usage_prc).truecolor(color[0], color[1], color[2])
    ); // prints the memory usage

    print!("{}", os_ascii[6]); // prints the seventh line of the ascii art
    if sys.total_swap() != 0 {
        // Checks if swap memory is available
        let swap_used_mb = sys.used_swap() / 1024 / 1024; // Converts used to MB
        let swap_total_mb = sys.total_swap() / 1024 / 1024; // Converts Total to MB
        let swap_usage_prc = sys.used_swap() * 100 / sys.total_swap(); // Calculates the percentage of swap memory used
        println!(
            "Swap Usage: {} {}",
            format!("{}/{} MB", swap_used_mb, swap_total_mb)
                .truecolor(color[0], color[1], color[2]),
            format!("({}%)", swap_usage_prc).truecolor(color[0], color[1], color[2])
        ); // prints the swap usage
    } else {
        println!(
            "Swap Usage: {}",
            "No Swap Memory".truecolor(color[0], color[1], color[2])
        ); // prints if no swap memory is available
    }

    print!("{}", os_ascii[7]); // prints the eighth line of the ascii art
    println!(
        "CPU Usage: {}",
        format!("{}%", sys.global_cpu_usage()).truecolor(color[0], color[1], color[2])
    ); // prints the CPU usage as a percentage
    
    Ok(())
}