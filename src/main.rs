use sysinfo::*;
use colored::Colorize;


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("=>System information");
    println!("System name: {}", System::name().unwrap().cyan());
    println!("Operating System Version: {}", System::long_os_version().unwrap_or_default().red());
    println!("System kernel version: {}", System::kernel_long_version());
    println!("Memory Usage : {}/{} MB ({}%)", sys.used_memory() / 1024 /1024, sys.total_memory() / 1024 / 1024, sys.used_memory() * 100 / sys.total_memory());
    if sys.total_swap() != 0 {
        println!("Swap Usage : {}/{} MB ({}%)", sys.used_swap() / 1024 /1024, sys.total_swap() / 1024 / 1024, sys.used_swap() * 100 / sys.total_swap());
    }
    else {
        println!("No swap memory available"); 
    }
    println!("CPU Usage : {}%", sys.global_cpu_usage());
    //println!("CPU Temperature : {}°C", sys.cpu_temperature().unwrap_or_default());
    //println!("Battery Usage: {}%", sys.used_battery())
    




   
    // println!(r"           .        ");
    // println!(r"          / \       ");
    // println!(r"         /   \      ");
    // println!(r"        /^.   \     ");
    // println!(r"       /  .-.  \    ");
    // println!(r"      /  (   ) _\   ");
    // println!(r"     / _.~   ~._^\  ");
    // println!(r"    /.^         ^.\ ");
}
