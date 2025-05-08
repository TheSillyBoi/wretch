use sysinfo::*;
use colored::Colorize;
//use std::string::String;
use colored::ColoredString;


fn ascii_art() -> [ColoredString; 8] {
    let os_version = System::long_os_version().unwrap_or_default();
    let mut retval: [ColoredString; 8] = Default::default();
    if os_version.contains("Arch"){
        retval[0] = ColoredString::from(r"           .        ").bright_cyan();
        retval[1] = ColoredString::from(r"          / \       ").bright_cyan();
        retval[2] = ColoredString::from(r"         /   \      ").bright_cyan();
        retval[3] = ColoredString::from(r"        /^.   \     ").bright_cyan();
        retval[4] = ColoredString::from(r"       /  .-.  \    ").bright_cyan();
        retval[5] = ColoredString::from(r"      /  (   ) _\   ").bright_cyan();
        retval[6] = ColoredString::from(r"     / _.~   ~._^\  ").bright_cyan();
        retval[7] = ColoredString::from(r"    /.^         ^.\ ").bright_cyan();
    }
    else if os_version.contains("Fedora"){
        retval[0] = ColoredString::from(r"    ▓▓▓▓▓▓▓▓▓▓▓▓    ").bright_cyan();
        retval[1] = ColoredString::from(r"  ▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓  ").bright_cyan();
        retval[2] = ColoredString::from(r" ▓▓▓▓▓▓▓▓  ▓▓  ▓▓▓▓ ").bright_cyan();
        retval[3] = ColoredString::from(r"▒▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓ ").bright_cyan();
        retval[4] = ColoredString::from(r"▓▓▓▓    ▓   ▓▓▓▓▓▓▓ ").bright_cyan();
        retval[5] = ColoredString::from(r"▓▓▓  ▓▓▓▓ ▓▓▓▓▓▓▓▓▓ ").bright_cyan();
        retval[6] = ColoredString::from(r"▓▓▓▓      ▓▓▓▓▓▓▓▓  ").bright_cyan();
        retval[7] = ColoredString::from(r"▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    ").bright_cyan();
    }
     else if os_version.contains("Windows"){

         retval[0] = ColoredString::from(r"  ╔══════╗  ╔══════╗").bright_cyan();
         retval[1] = ColoredString::from(r"  ║      ║  ║      ║").bright_cyan();
         retval[2] = ColoredString::from(r"  ║      ║  ║      ║").bright_cyan();
         retval[3] = ColoredString::from(r"  ╚══════╝  ╚══════╝").bright_cyan();
         retval[4] = ColoredString::from(r"  ╔══════╗  ╔══════╗").bright_cyan();
         retval[5] = ColoredString::from(r"  ║      ║  ║      ║").bright_cyan();
         retval[6] = ColoredString::from(r"  ║      ║  ║      ║").bright_cyan();
         retval[7] = ColoredString::from(r"  ╚══════╝  ╚══════╝").bright_cyan();
     }

    return retval;
}
fn main() {
    let os_ascii = ascii_art();
    let mut sys = System::new_all();
    sys.refresh_all();

    print!("{}",os_ascii[0].clone());
    println!("System information");

    print!("{}",os_ascii[1].clone());
    println!("System name: {}", System::name().unwrap().cyan());

    print!("{}",os_ascii[2].clone());
    println!("Operating System Version: {}", System::long_os_version().unwrap_or_default().cyan());

    print!("{}",os_ascii[3].clone());
    println!("System kernel version: {}", System::kernel_long_version().cyan());

    print!("{}",os_ascii[4].clone());
    println!("Hostname: {}", System::host_name().unwrap().cyan());


    let mem_used_mb = sys.used_memory() / 1024 /1024;
    let mem_total_mb = sys.total_memory() / 1024 / 1024;
    let mem_usage_prc =  sys.used_memory() * 100 / sys.total_memory();
    let swap_used_mb = sys.used_swap() / 1024 /1024;
    let swap_total_mb = sys.total_swap() / 1024 /1024;
    let swap_usage_prc = sys.used_swap() * 100 / sys.total_swap();
    print!("{}",os_ascii[5].clone().bright_cyan());
    println!("Memory Usage : {}{}{} {} {}{}{}", mem_used_mb.to_string().cyan(), "/".cyan(), mem_total_mb.to_string().cyan(), "MB".cyan(), "(".cyan() ,mem_usage_prc.to_string().cyan(),"%)".cyan());

    print!("{}",os_ascii[6].clone());
    if sys.total_swap() != 0 {
        println!("Swap Usage : {}{}{} {} {}{}{}",swap_used_mb.to_string().cyan(), "/".cyan() , swap_total_mb.to_string().cyan(), "MB".cyan(),"(".cyan(), swap_usage_prc.to_string().cyan()  , "%)".cyan());
    }

    else {
        println!("No Swap Memory"); 
    }

    print!("{}",os_ascii[7].clone());
    println!("CPU Usage : {}{}", sys.global_cpu_usage().to_string().cyan(),"%".cyan());
    // println!("CPU Temperature : {}°C", sys.cpu_temperature().unwrap_or_default());
    // println!("Battery Usage: {}%"

}