use sysinfo::System;
use colored::{Colorize,ColoredString};


fn ascii_art() -> [ColoredString; 8] {
    let os_version = System::long_os_version().unwrap_or_default().to_lowercase();
    let mut retval: [ColoredString; 8] = Default::default();
    if os_version.contains("arch"){
        retval[0] = ColoredString::from(r"           .        ").bright_cyan();
        retval[1] = ColoredString::from(r"          / \       ").bright_cyan();
        retval[2] = ColoredString::from(r"         /   \      ").bright_cyan();
        retval[3] = ColoredString::from(r"        /^.   \     ").bright_cyan();
        retval[4] = ColoredString::from(r"       /  .-.  \    ").bright_cyan();
        retval[5] = ColoredString::from(r"      /  (   ) _\   ").bright_cyan();
        retval[6] = ColoredString::from(r"     / _.~   ~._^\  ").bright_cyan();
        retval[7] = ColoredString::from(r"    /.^         ^.\ ").bright_cyan();
    } else if os_version.contains("fedora"){
        retval[0] = ColoredString::from(r"    ▓▓▓▓▓▓▓▓▓▓▓▓    ").bright_cyan();
        retval[1] = ColoredString::from(r"  ▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓  ").bright_cyan();
        retval[2] = ColoredString::from(r" ▓▓▓▓▓▓▓▓  ▓▓  ▓▓▓▓ ").bright_cyan();
        retval[3] = ColoredString::from(r"▒▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓ ").bright_cyan();
        retval[4] = ColoredString::from(r"▓▓▓▓    ▓   ▓▓▓▓▓▓▓ ").bright_cyan();
        retval[5] = ColoredString::from(r"▓▓▓  ▓▓▓▓ ▓▓▓▓▓▓▓▓▓ ").bright_cyan();
        retval[6] = ColoredString::from(r"▓▓▓▓      ▓▓▓▓▓▓▓▓  ").bright_cyan();
        retval[7] = ColoredString::from(r"▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    ").bright_cyan();
    } else if os_version.contains("windows"){
        retval[0] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 120, 212);
        retval[1] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[2] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[3] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 120, 212);
        retval[4] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 120, 212);
        retval[5] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[6] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[7] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 120, 212);
    } else if os_version.contains("mac"){
        retval[0] = ColoredString::from(r"           ▓▓       ").white();
        retval[1] = ColoredString::from(r"          ▓▓        ").white();
        retval[2] = ColoredString::from(r"     ▓▓▓▓    ▓▓▓    ").white();
        retval[3] = ColoredString::from(r"    ▓▓▓▓▓▓▓▓▓▓▓     ").white();
        retval[4] = ColoredString::from(r"   ▓▓▓▓▓▓▓▓▓▓▓      ").white();
        retval[5] = ColoredString::from(r"   ▓▓▓▓▓▓▓▓▓▓▓▓     ").white();
        retval[6] = ColoredString::from(r"    ▓▓▓▓▓▓▓▓▓▓▓▓▓   ").white();
        retval[7] = ColoredString::from(r"      ▓▓▓    ▓▓▓    ").white();
    } else if os_version.contains("ubuntu"){
        retval[0] = ColoredString::from(r"▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ▓▓▓").truecolor(250, 70, 22);
        retval[1] = ColoredString::from(r"▓▓▓▓▓  ▓▓     ▓▓▓▓▓▓").truecolor(250, 70, 22);
        retval[2] = ColoredString::from(r"▓▓▓▓  ▓▓▓▓▓▓▓   ▓▓▓▓").truecolor(250, 70, 22);
        retval[3] = ColoredString::from(r"  ▓  ▓▓▓    ▓▓   ▓▓▓").truecolor(250, 70, 22);
        retval[4] = ColoredString::from(r"▓▓▓  ▓▓▓    ▓▓▓▓▓▓▓▓").truecolor(250, 70, 22);
        retval[5] = ColoredString::from(r"▓▓▓▓  ▓▓▓▓▓▓▓   ▓▓▓▓").truecolor(250, 70, 22);
        retval[6] = ColoredString::from(r"▓▓▓▓▓▓▓▓      ▓▓▓▓▓▓").truecolor(250, 70, 22);
        retval[7] = ColoredString::from(r"▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓").truecolor(250, 70, 22);
    } else if os_version.contains("linux"){
        retval[0] = ColoredString::from(r"        .---.       ");
        retval[1] = ColoredString::from(r"       /     \      ");
        retval[2] = ColoredString::from(r"       \.o-o./      ");
        retval[3] = ColoredString::from(r"       /`\_/`\      ");
        retval[4] = ColoredString::from(r"      //     \\     ");
        retval[5] = ColoredString::from(r"     | \     )|_    ");
        retval[6] = ColoredString::from(r"    /`\_`>  <_/ \   ");
        retval[7] = ColoredString::from(r" jgs\__/'---'\__/   ");
      
    }

    retval
}


fn main() {
    let os_ascii = ascii_art();
    let mut sys = System::new_all();
    sys.refresh_all();

    

    print!("{}", os_ascii[0]);
    println!("{}'s System information", whoami::realname());

    print!("{}", os_ascii[1]);
    println!("System name: {} {}", System::name().unwrap().cyan(), format!("({})", System::cpu_arch()).cyan());

    print!("{}", os_ascii[2]);
    println!("Operating System Version: {}", System::long_os_version().unwrap_or_default().cyan());

    print!("{}", os_ascii[3]);
    println!("System kernel version: {}", System::kernel_long_version().cyan());

    print!("{}", os_ascii[4]);
    println!("Hostname: {}", format!("{}@{}", whoami::username(), System::host_name().unwrap()).cyan());


    let mem_used_mb = sys.used_memory() / 1024 /1024; //Converts used to GB
    let mem_total_mb = sys.total_memory() / 1024 / 1024; //Converts Total to GB
    let mem_usage_prc =  sys.used_memory() * 100 / sys.total_memory();
    print!("{}", os_ascii[5]);
    println!("Memory Usage: {} {}", format!("{}/{} MB", mem_used_mb.to_string(), mem_total_mb.to_string().cyan()).cyan() , format!("({}%)", mem_usage_prc.to_string()).cyan());

    print!("{}", os_ascii[6]);
    if sys.total_swap() != 0 {
        let swap_used_mb = sys.used_swap() / 1024 / 1024;
        let swap_total_mb = sys.total_swap() / 1024 / 1024;
        let swap_usage_prc = sys.used_swap() * 100 / sys.total_swap();
        println!("Swap Usage: {} {}", format!("{}/{} MB", swap_used_mb.to_string(), swap_total_mb.to_string()).cyan(), format!("({}%)", swap_usage_prc.to_string()).cyan());
    } else {
        println!("Swap Usage: {}", "No Swap Memory (0%)".cyan()); 
    }

    print!("{}", os_ascii[7]);
    println!("CPU Usage: {}", format!("{}%", sys.global_cpu_usage().to_string()).cyan());
}
