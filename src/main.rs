use clap::Parser; // Import the Parser trait from clap
use sysinfo::System; // Import the System struct from sysinfo
use colored::Colorize; //import the Colorize trait and ColoredString struct from colored
mod ascii; // Import the ascii module(ascii.rs)



#[derive(Parser, Debug)]
#[command(name = "wretch", version = build_version(), about = "the tool to get information about your system", ignore_errors(true))]
struct Args {
    ascii: Option<String>, // Argument to choose what ASCII art to use, currently unfinished


}
pub fn ascii_name() -> String {
    if Args::parse().ascii.is_some() { // Check if the ascii argument is provided
        let os_ascii_name = Args::parse().ascii.unwrap(); // Get the ASCII name from the command line argument
        return os_ascii_name; // Return the ASCII name
    }
    else { // If the ascii argument is not provided
    let os_ascii_name = System::long_os_version().unwrap_or_default().to_lowercase(); // Get the OS version and convert it to lowercase
    let os_ascii_name = os_ascii_name.replace(" ", ""); // Remove spaces from the OS version string
    let os_ascii_name = os_ascii_name.replace("(", ""); // Remove parentheses from the OS version string
    let os_ascii_name = os_ascii_name.replace(")", ""); // Remove parentheses from the OS version string
    return os_ascii_name; // Return the cleaned OS version string}
    }

}
fn main() {
    

    let _args = Args::parse(); // Parse the command line arguments
    let os_ascii = ascii::ascii_art(); // Calls the ascii_art function to get the ASCII art based on the OS
    
    let mut sys = System::new_all(); // Gather system information
    sys.refresh_all(); // Refresh all system information


    

    print!("{}", os_ascii[0]); // prints the first line of the ascii art
    println!("{}'s System information", whoami::realname()); // Prints the person's name using the whoami crate

    print!("{}", os_ascii[1]); // prints the second line of the ascii art
    println!("System name: {} {}", System::name().unwrap().cyan(), format!("({})", System::cpu_arch()).cyan()); // Prints the system name and CPU architecture

    print!("{}", os_ascii[2]); // prints the third line of the ascii art
    println!("Operating System Version: {}", System::long_os_version().unwrap_or_default().cyan()); // Prints the OS version

    print!("{}", os_ascii[3]); // prints the fourth line of the ascii art
    println!("System kernel version: {}", System::kernel_long_version().cyan()); // Prints the kernel version

    print!("{}", os_ascii[4]);// prints the fifth line of the ascii art
    println!("Hostname: {}", format!("{}@{}", whoami::username(), System::host_name().unwrap()).cyan()); // Prints the hostname and username


    let mem_used_mb = sys.used_memory() / 1024 / 1024; // Converts used to MB
    let mem_total_mb = sys.total_memory() / 1024 / 1024; // Converts Total to MB
    let mem_usage_prc =  sys.used_memory() * 100 / sys.total_memory(); // Calculates the percentage of memory used
    print!("{}", os_ascii[5]); // prints the sixth line of the ascii art
    println!("Memory Usage: {} {}", format!("{}/{} MB", mem_used_mb, mem_total_mb).cyan() , format!("({}%)", mem_usage_prc).cyan()); // prints the memory usage

    print!("{}", os_ascii[6]);// prints the seventh line of the ascii art
    if sys.total_swap() != 0 { // Checks if swap memory is available
        let swap_used_mb = sys.used_swap() / 1024 / 1024; // Converts used to MB
        let swap_total_mb = sys.total_swap() / 1024 / 1024; // Converts Total to MB
        let swap_usage_prc = sys.used_swap() * 100 / sys.total_swap(); // Calculates the percentage of swap memory used
        println!("Swap Usage: {} {}", format!("{}/{} MB", swap_used_mb, swap_total_mb).cyan(), format!("({}%)", swap_usage_prc).cyan()); // prints the swap usage
    } else {
        println!("Swap Usage: {}", "No Swap Memory".cyan()); // prints if no swap memory is available
    }

    print!("{}", os_ascii[7]);// prints the eighth line of the ascii art
    println!("CPU Usage: {}", format!("{}%", sys.global_cpu_usage()).cyan()); // prints the CPU usage as a percentage
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
        if commit != "" {
            format!("{base}-nightly ({commit})")
        } else {
            format!("{base}-nightly (commit unknown)")
        }
    } else if commit != "" {
            format!("{base} ({commit})")
    } else {
            format!("{base} (commit unknown)")
        }
}
