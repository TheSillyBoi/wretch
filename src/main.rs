use clap::Parser; // Import the Parser trait from clap
use sysinfo::System; // Import the System struct from sysinfo
use colored::{Colorize,ColoredString}; //import the Colorize trait and ColoredString struct from colored


fn ascii_art() -> [ColoredString; 8] { // Function to Select an ASCII art based on the OS
    let os_version = System::long_os_version().unwrap_or_default().to_lowercase(); // Get the OS version and convert it to lowercase
    let mut retval: [ColoredString; 8] = Default::default(); // Initialize an array of ColoredString with 8 elements
    if os_version.contains("arch"){ // if the OS version contains "arch"(meant to be used for arch linux)
        retval[0] = ColoredString::from(r"           .        ").bright_cyan();
        retval[1] = ColoredString::from(r"          / \       ").bright_cyan();
        retval[2] = ColoredString::from(r"         /   \      ").bright_cyan();
        retval[3] = ColoredString::from(r"        /^.   \     ").bright_cyan();
        retval[4] = ColoredString::from(r"       /  .-.  \    ").bright_cyan();
        retval[5] = ColoredString::from(r"      /  (   ) _\   ").bright_cyan();
        retval[6] = ColoredString::from(r"     / _.~   ~._^\  ").bright_cyan();
        retval[7] = ColoredString::from(r"    /.^         ^.\ ").bright_cyan();
    } else if os_version.contains("fedora"){ // if the OS version contains "fedora" meant for fedora linux
        retval[0] = ColoredString::from(r"    ╔══════════╗    ").bright_cyan();
        retval[1] = ColoredString::from(r"  ╔═╝          ╚═╗  ").bright_cyan();
        retval[2] = ColoredString::from(r" ╔╝     ╔═══╗    ╚╗ ").bright_cyan();
        retval[3] = ColoredString::from(r"╔╝      ║         ║ ").bright_cyan();
        retval[4] = ColoredString::from(r"║  ╔═══ ╠═══      ║ ").bright_cyan();
        retval[5] = ColoredString::from(r"║  ║    ║        ╔╝ ").bright_cyan();
        retval[6] = ColoredString::from(r"║  ╚════╝      ╔═╝  ").bright_cyan();
        retval[7] = ColoredString::from(r"╚══════════════╝    ").bright_cyan();
    } else if os_version.contains("windows"){ // if the OS version contains "windows" meant for windows
        retval[0] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 120, 212);
        retval[1] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[2] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[3] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 120, 212);
        retval[4] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 120, 212);
        retval[5] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[6] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[7] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 120, 212);
    } else if os_version.contains("mac"){ // if the OS version contains "mac" meant for macOS
        retval[0] = ColoredString::from(r"           ╔═       ").white();
        retval[1] = ColoredString::from(r"          ╔╝        ").white();
        retval[2] = ColoredString::from(r"   ╔════╗   ╔═══╗   ").white();
        retval[3] = ColoredString::from(r"  ╔╝    ╚═══╝  ╔╝   ").white();
        retval[4] = ColoredString::from(r"  ║           ╔╝    ").white();
        retval[5] = ColoredString::from(r"  ║           ╚╗    ").white();
        retval[6] = ColoredString::from(r"  ╚╗   ╔════╗  ╚╗   ").white();
        retval[7] = ColoredString::from(r"   ╚═══╝    ╚═══╝   ").white();
    } else if os_version.contains("debian"){ // if the OS version contains "debian" meant for debian linux, to do
        retval[0] = ColoredString::from(r"        ╔═════╗     ").red();
        retval[1] = ColoredString::from(r"      ╔═╝╔══╗ ║     ").red();
        retval[2] = ColoredString::from(r"     ╔╝  ║ ╚╝╔╝     ").red();  
        retval[3] = ColoredString::from(r"     ║   ╚═══╝      ").red(); 
        retval[4] = ColoredString::from(r"     ╚╗             ").red(); 
        retval[5] = ColoredString::from(r"      ╚═╗           ").red(); 
        retval[6] = ColoredString::from(r"        ╚═╗         ").red(); 
        retval[7] = ColoredString::from(r"          ╚═        ").red();
    } else if os_version.contains("void"){ // if the OS version contains "void" meant for void linux
        retval[0] = ColoredString::from(r"       ╔═══════╗    ").green();
        retval[1] = ColoredString::from(r"       ╚═════╗ ╚═╗  ").green();
        retval[2] = ColoredString::from(r"             ╚═╗ ║  ").green();
        retval[3] = ColoredString::from(r"   ╔═╗   ╔═╗   ║ ║  ").green();
        retval[4] = ColoredString::from(r"   ║ ║   ╚═╝   ╚═╝  ").green();
        retval[5] = ColoredString::from(r"   ║ ╚═╗            ").green();
        retval[6] = ColoredString::from(r"   ╚═╗ ╚═════╗      ").green();
        retval[7] = ColoredString::from(r"     ╚═══════╝      ").green();
    } else if os_version.contains("suse"){ // if the OS version contains "suse" meant for opensuse linux
        retval[0] = ColoredString::from(r"      ╔═══════╗     ").green();
        retval[1] = ColoredString::from(r"    ╔═╝╔══╗   ╚═╗   ").green();
        retval[2] = ColoredString::from(r"  ╔═╩══╝  ╚═══╗ ╚═╗ ").green();
        retval[3] = ColoredString::from(r"  ║        ╔═╗╚╗  ║ ").green();
        retval[4] = ColoredString::from(r"  ║        ╚═╝╔╝  ║ ").green();
        retval[5] = ColoredString::from(r"  ╚═╗   ╔═════╝ ╔═╝ ").green();
        retval[6] = ColoredString::from(r"    ╚═╦═╩════ ╔═╝   ").green();
        retval[7] = ColoredString::from(r"      ╚═══════╝     ").green();
    } else if os_version.contains("ubuntu"){ // if the OS version contains "ubuntu" meant for ubuntu linux
        retval[0] = ColoredString::from(r"             ╔═╗    ").truecolor(250, 70, 22);
        retval[1] = ColoredString::from(r"             ╚═╝    ").truecolor(250, 70, 22);
        retval[2] = ColoredString::from(r"    ╔══   ════╗     ").truecolor(250, 70, 22);
        retval[3] = ColoredString::from(r"╔═╗ ║  ╔════╗ ║     ").truecolor(250, 70, 22);
        retval[4] = ColoredString::from(r"╚═╝ ║  ║    ║ ║     ").truecolor(250, 70, 22);
        retval[5] = ColoredString::from(r"    ║  ╚════╝       ").truecolor(250, 70, 22);
        retval[6] = ColoredString::from(r"      ════════╝ ╔═╗ ").truecolor(250, 70, 22);
        retval[7] = ColoredString::from(r"                ╚═╝ ").truecolor(250, 70, 22);
    } else if os_version.contains("zorin"){ // if the OS version contains "zorin" meant for zorinOS linux
        retval[0] = ColoredString::from(r"    ____________    ").truecolor(250, 70, 22);
        retval[1] = ColoredString::from(r"   /____________\   ").truecolor(250, 70, 22);
        retval[2] = ColoredString::from(r"  _________         ").truecolor(250, 70, 22);
        retval[3] = ColoredString::from(r" /                \ ").truecolor(250, 70, 22);
        retval[4] = ColoredString::from(r" \       _________/ ").truecolor(250, 70, 22);
        retval[5] = ColoredString::from(r"   ______________   ").truecolor(250, 70, 22);
        retval[6] = ColoredString::from(r"   \____________/   ").truecolor(250, 70, 22);
        retval[7] = ColoredString::from(r"                    ").truecolor(250, 70, 22);
    } else if os_version.contains("linux"){ // if the OS version contains "linux" meant for other linux distros
        retval[0] = ColoredString::from(r"        .---.       ");
        retval[1] = ColoredString::from(r"       /     \      ");
        retval[2] = ColoredString::from(r"       \.o-o./      ");
        retval[3] = ColoredString::from(r"       /`\_/`\      ");
        retval[4] = ColoredString::from(r"      //     \\     ");
        retval[5] = ColoredString::from(r"     | \     )|_    ");
        retval[6] = ColoredString::from(r"    /`\_`>  <_/ \   ");
        retval[7] = ColoredString::from(r" jgs\__/'---'\__/   ");
    } else { // if the OS version does not match any of the above
        retval[0] = ColoredString::from(r"╔══════════════════╗").truecolor(255,255,255);
        retval[1] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[2] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[3] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[4] = ColoredString::from(r"╚═══════╦══╦═══════╝").truecolor(255,255,255);
        retval[5] = ColoredString::from(r"        ║  ║        ").truecolor(255,255,255);
        retval[6] = ColoredString::from(r"    ╔═══╝  ╚═══╗    ").truecolor(255,255,255);
        retval[7] = ColoredString::from(r"    ╚══════════╝    ").truecolor(255,255,255);
      
    }

    retval
}


#[derive(Parser, Debug)]
#[command(name = "wretch", version = build_version(), about = "the tool to get information about your system", ignore_errors(true))]
struct Args {}

fn main() {
    let os_ascii = ascii_art(); // Calls the ascii_art function to get the ASCII art based on the OS
    let mut sys = System::new_all(); // Gather system information
    sys.refresh_all(); // Refresh all system information

    let _args = Args::parse();

    

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
    println!("CPU Usage: {}", format!("{}%", sys.global_cpu_usage().to_string()).cyan()); // prints the CPU usage as a percentage
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
    } else {
        if commit != "" {
            format!("{base} ({commit})")
        } else {
            format!("{base} (commit unknown)")
        }
    }
}
