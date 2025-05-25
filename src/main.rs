use clap::Parser; // Import the Parser trait from clap
use colored::Colorize;
use sysinfo::System; // Import the System struct from sysinfo //import the Colorize trait and ColoredString struct from colored
mod ascii; // Import the ascii module(ascii.rs)
use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

#[derive(Parser, Debug)]
#[command(name = "wretch", version = build_version(), about = "the tool to get information about your system", ignore_errors(true))]
struct Args {
    ascii: Option<String>, // Argument to choose what ASCII art to use, I need to make it as a flag, but idk how to do that
}

fn get_os_name() -> (Option<String>, Option<String>) {
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

pub fn ascii_name() -> String {
    static CACHE: OnceLock<String> = OnceLock::new();

    if let Some(cached) = CACHE.get() {
        return cached.clone();
    }

    let args = Args::parse();

    let result = if let Some(ascii_arg) = args.ascii {
        ascii_arg
    } else {
        let (id_opt, id_like_opt) = get_os_name();

        if let Some(ref id) = id_opt {
            let (_, generic) = ascii::ascii_art(id);
            if !generic {
                id.clone()
            } else if let Some(ref id_like) = id_like_opt {
                id_like.clone()
            } else {
                let mut os_ascii_name =
                    System::long_os_version().unwrap_or_default().to_lowercase();
                os_ascii_name = os_ascii_name.replace(' ', "");
                os_ascii_name = os_ascii_name.replace('(', "");
                os_ascii_name = os_ascii_name.replace(')', "");
                os_ascii_name
            }
        } else if let Some(ref id_like) = id_like_opt {
            id_like.clone()
        } else {
            let mut os_ascii_name = System::long_os_version().unwrap_or_default().to_lowercase();
            os_ascii_name = os_ascii_name.replace(' ', "");
            os_ascii_name = os_ascii_name.replace('(', "");
            os_ascii_name = os_ascii_name.replace(')', "");
            os_ascii_name
        }
    };

    CACHE.set(result.clone()).ok();

    result
}

pub fn info_color() -> [u8; 3] {
    // Function to get the color values based on the OS for the Info
    let mut color = [1, 2, 3];
    if ascii_name().contains("fedora")
        || ascii_name().contains("nixos")
        || ascii_name().contains("arch")
    {
        color[0] = 0;
        color[1] = 120;
        color[2] = 212;
    } else if ascii_name().contains("windows") || ascii_name().contains("zorin") {
        color[0] = 0;
        color[1] = 0;
        color[2] = 250;
    } else if ascii_name().contains("ubuntu") {
        color[0] = 250;
        color[1] = 70;
        color[2] = 22;
    } else if ascii_name().contains("mac") {
        color[0] = 255;
        color[1] = 255;
        color[2] = 255;
    } else if ascii_name().contains("debian") {
        color[0] = 255;
        color[1] = 0;
        color[2] = 0;
    } else if ascii_name().contains("void") || ascii_name().contains("suse") {
        color[0] = 0;
        color[1] = 255;
        color[2] = 0;
    } else {
        color[0] = 255;
        color[1] = 255;
        color[2] = 255;
    }
    color
}

fn main() {
    let color = info_color(); // Calls the info_color function to get the color values
    let _args = Args::parse(); // Parse the command line arguments
    let (os_ascii, _) = ascii::ascii_art(&ascii_name()); // Calls the ascii_art function to get the ASCII art based on the OS

    let mut sys = System::new_all(); // Gather system information
    sys.refresh_all(); // Refresh all system information

    print!("{}", os_ascii[0]); // prints the first line of the ascii art
    println!("{}'s System information", whoami::realname()); // Prints the person's name using the whoami crate
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
}

// Versioning
fn build_version() -> &'static str {
    Box::leak(format_version().into_boxed_str())
}

fn format_version() -> String {
    let base = env!("CARGO_PKG_VERSION");
    let commit = env!("GIT_HASH");
    let is_nightly = env!("NIGHTLY");

    if is_nightly == "true" {
        if !commit.is_empty() {
            format!("{base}-nightly ({commit})")
        } else {
            format!("{base}-nightly (commit unknown)")
        }
    } else if !commit.is_empty() {
        format!("{base} ({commit})")
    } else {
        format!("{base} (commit unknown)")
    }
}
