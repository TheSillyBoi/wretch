use sysinfo::*;
use colored::Colorize;
use std::string::String;


fn ascii_art() -> [String; 8] {
    let os_version = System::long_os_version().unwrap_or_default();
    let mut retval: [String; 8] = Default::default();
    if os_version.contains("Arch"){
        retval[0] = String::from(r"           .        ");
        retval[1] = String::from(r"          / \       ");
        retval[2] = String::from(r"         /   \      ");
        retval[3] = String::from(r"        /^.   \     ");
        retval[4] = String::from(r"       /  .-.  \    ");
        retval[5] = String::from(r"      /  (   ) _\   ");
        retval[6] = String::from(r"     / _.~   ~._^\  ");
        retval[7] = String::from(r"    /.^         ^.\ ");
    }
    else if os_version.contains("Fedora"){
        retval[0] = String::from(r"    ▓▓▓▓▓▓▓▓▓▓▓▓    ");
        retval[1] = String::from(r"  ▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓  ");
        retval[2] = String::from(r" ▓▓▓▓▓▓▓▓ ░▓▓  ▓▓▓▓ ");
        retval[3] = String::from(r"▒▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓ ");
        retval[4] = String::from(r"▓▓▓▓    ▓   ▓▓▓▓▓▓▓ ");
        retval[5] = String::from(r"▓▓▓  ▓▓▓▓ ▓▓▓▓▓▓▓▓▓ ");
        retval[6] = String::from(r"▓▓▓▓  ▓▒  ▓▓▓▓▓▓▓▓  ");
        retval[7] = String::from(r"▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    ");
    }
     else if os_version.contains("Windows"){

         retval[0] = String::from(r"#######    ####### ");
         retval[1] = String::from(r"#######    ####### ");
         retval[2] = String::from(r"#######    ####### ");
         retval[3] = String::from(r"                   ");
         retval[4] = String::from(r"                   ");
         retval[5] = String::from(r"#######    ####### ");
         retval[6] = String::from(r"#######    ####### ");
         retval[7] = String::from(r"#######    ####### ");
     }

    return retval;
}
fn main() {
    let os_ascii = ascii_art();
    let mut sys = System::new_all();
    sys.refresh_all();

    print!("{}",os_ascii[0].bright_cyan());
    println!("System information");

    print!("{}",os_ascii[1].bright_cyan());
    println!("System name: {}", System::name().unwrap().cyan());

    print!("{}",os_ascii[2].bright_cyan());
    println!("Operating System Version: {}", System::long_os_version().unwrap_or_default().cyan());

    print!("{}",os_ascii[3].bright_cyan());
    println!("System kernel version: {}", System::kernel_long_version().cyan());

    print!("{}",os_ascii[4].bright_cyan());
    println!("Hostname: {}", System::host_name().unwrap().cyan());


    let mem_used_mb = sys.used_memory() / 1024 /1024;
    let mem_total_mb = sys.total_memory() / 1024 / 1024;
    print!("{}",os_ascii[5].bright_cyan());
    println!("Memory Usage : {}/{} MB ({}{}", mem_used_mb.to_string().cyan(), mem_total_mb.to_string().cyan() , sys.used_memory() * 100 / sys.total_memory(),"%)".cyan());

    print!("{}",os_ascii[6].bright_cyan());
    if sys.total_swap() != 0 {
        println!("Swap Usage : {}/{} MB ({}%)", sys.used_swap() / 1024 /1024, sys.total_swap() / 1024 / 1024, sys.used_swap() * 100 / sys.total_swap());
    }

    else {
        println!("No swap memory available"); 
    }

    print!("{}",os_ascii[7].bright_cyan());
    println!("CPU Usage : {}{}", sys.global_cpu_usage().to_string().cyan(),"%".cyan());
    // println!("CPU Temperature : {}°C", sys.cpu_temperature().unwrap_or_default());
    // println!("Battery Usage: {}%", sys.used_battery())
    
    // for line in os_ascii.iter() {
    //     println!("{}", line.bright_cyan());
    // }

}

