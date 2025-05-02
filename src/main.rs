use sysinfo::*;
use colored::Colorize;

fn ascii_art(){
    if System::long_os_version().unwrap_or_default().contains("Fedora"){
        // let _arch_ascii1 = r"           .        ";
        // let _arch_ascii2 = r"          / \       ";
        // let _arch_ascii3 = r"         /   \      ";
        // let _arch_ascii4 = r"        /^.   \     ";
        // let _arch_ascii5 = r"       /  .-.  \    ";
        // let _arch_ascii6 = r"      /  (   ) _\   ";
        // let _arch_ascii7 = r"     / _.~   ~._^\  ";
        // let _arch_ascii8 = r"    /.^         ^.\ ";
    }
        


}
fn main() {

    ascii_art();
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
    
    // println!("{}",_arch_ascii1);
    // println!("{}",_arch_ascii2);
    // println!("{}",_arch_ascii3);
    // println!("{}",_arch_ascii4);
    // println!("{}",_arch_ascii5);
    // println!("{}",_arch_ascii6);
    // println!("{}",_arch_ascii7);
    // println!("{}",_arch_ascii8);




}

