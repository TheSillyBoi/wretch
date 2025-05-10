use sysinfo::{System};
use colored::{Colorize,ColoredString};


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
    } else if os_version.contains("Fedora"){
        retval[0] = ColoredString::from(r"    ▓▓▓▓▓▓▓▓▓▓▓▓    ").bright_cyan();
        retval[1] = ColoredString::from(r"  ▓▓▓▓▓▓▓▓▓  ▓▓▓▓▓  ").bright_cyan();
        retval[2] = ColoredString::from(r" ▓▓▓▓▓▓▓▓  ▓▓  ▓▓▓▓ ").bright_cyan();
        retval[3] = ColoredString::from(r"▒▓▓▓▓▓▓▓▓ ▓▓▓▓▓▓▓▓▓ ").bright_cyan();
        retval[4] = ColoredString::from(r"▓▓▓▓    ▓   ▓▓▓▓▓▓▓ ").bright_cyan();
        retval[5] = ColoredString::from(r"▓▓▓  ▓▓▓▓ ▓▓▓▓▓▓▓▓▓ ").bright_cyan();
        retval[6] = ColoredString::from(r"▓▓▓▓      ▓▓▓▓▓▓▓▓  ").bright_cyan();
        retval[7] = ColoredString::from(r"▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    ").bright_cyan();
    } else if os_version.contains("Windows"){
        retval[0] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 120, 212);
        retval[1] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[2] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[3] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 120, 212);
        retval[4] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 120, 212);
        retval[5] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[6] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[7] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 120, 212);
    } else if os_version.contains("Mac"){
        retval[0] = ColoredString::from(r"           ▓▓       ").white();
        retval[1] = ColoredString::from(r"          ▓▓        ").white();
        retval[2] = ColoredString::from(r"     ▓▓▓▓    ▓▓▓    ").white();
        retval[3] = ColoredString::from(r"    ▓▓▓▓▓▓▓▓▓▓▓     ").white();
        retval[4] = ColoredString::from(r"   ▓▓▓▓▓▓▓▓▓▓▓      ").white();
        retval[5] = ColoredString::from(r"   ▓▓▓▓▓▓▓▓▓▓▓▓     ").white();
        retval[6] = ColoredString::from(r"    ▓▓▓▓▓▓▓▓▓▓▓▓▓   ").white();
        retval[7] = ColoredString::from(r"      ▓▓▓    ▓▓▓    ").white();
    } else if os_version.contains("Ubuntu"){
        retval[0] = ColoredString::from(r"▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ▓▓▓").truecolor(250, 70, 22);
        retval[1] = ColoredString::from(r"▓▓▓▓▓  ▓▓     ▓▓▓▓▓▓").truecolor(250, 70, 22);
        retval[2] = ColoredString::from(r"▓▓▓▓  ▓▓▓▓▓▓▓   ▓▓▓▓").truecolor(250, 70, 22);
        retval[3] = ColoredString::from(r"  ▓  ▓▓▓    ▓▓   ▓▓▓").truecolor(250, 70, 22);
        retval[4] = ColoredString::from(r"▓▓▓  ▓▓▓    ▓▓▓▓▓▓▓▓").truecolor(250, 70, 22);
        retval[5] = ColoredString::from(r"▓▓▓▓  ▓▓▓▓▓▓▓   ▓▓▓▓").truecolor(250, 70, 22);
        retval[6] = ColoredString::from(r"▓▓▓▓▓▓▓▓      ▓▓▓▓▓▓").truecolor(250, 70, 22);
        retval[7] = ColoredString::from(r"▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ▓▓▓▓").truecolor(250, 70, 22);
    } else if os_version.contains("Linux"){
        retval[0] = ColoredString::from(r"        .---.      ");
        retval[1] = ColoredString::from(r"       /     \     ");
        retval[2] = ColoredString::from(r"       \.o-o./     ");
        retval[3] = ColoredString::from(r"       /`\_/`\     ");
        retval[4] = ColoredString::from(r"      //     \\    ");
        retval[5] = ColoredString::from(r"     | \     )|_   ");
        retval[6] = ColoredString::from(r"    /`\_`>  <_/ \  ");
        retval[7] = ColoredString::from(r" jgs\__/'---'\__/  ");
      
    }

    return retval;
}
fn main() {
    let os_ascii: [ColoredString; 8] = ascii_art();
    let mut sys: System = System::new_all();
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


    let mem_used_mb: u64 = sys.used_memory() / 1024 /1024;
    let mem_total_mb: u64 = sys.total_memory() / 1024 / 1024;
    let mem_usage_prc: u64 =  sys.used_memory() * 100 / sys.total_memory();
    let swap_used_mb: u64 = sys.used_swap() / 1024 / 1024;
    let swap_total_mb: u64 = sys.total_swap() / 1024 / 1024;
    let swap_usage_prc: u64 = sys.used_swap() * 100 / sys.total_swap();
    print!("{}",os_ascii[5].clone());
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


}