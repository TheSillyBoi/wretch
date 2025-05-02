use sysinfo::*;
use colored::Colorize;

fn ascii_art() -> [String; 8] {
    let retval: [String; 8] = [""; 8];
    if System::long_os_version().unwrap_or_default().contains("Arch"){
        retval[0] = String::from(r"           .        ");
        // let _ascii2 = r"          / \       ";
        // let _ascii3 = r"         /   \      ";
        // let _ascii4 = r"        /^.   \     ";
        // let _ascii5 = r"       /  .-.  \    ";
        // let _ascii6 = r"      /  (   ) _\   ";
        // let _ascii7 = r"     / _.~   ~._^\  ";
        // let _ascii8 = r"    /.^         ^.\ ";
    }
    else if System::long_os_version().unwrap_or_default().contains("Fedora"){
        retval[0] = String::from(r"           .        ");
        retval[1] = String::from(r"          / \       ");
        retval[2] = String::from(r"         /   \      ");
        retval[3] = String::from(r"        /^.   \     ");
        retval[4] = String::from(r"       /  .-.  \    ");
        retval[5] = String::from(r"      /  (   ) _\   ");
        retval[6] = String::from(r"     / _.~   ~._^\  ");
        retval[7] = String::from(r"    /.^         ^.\ ");
    }
    // else if System::long_os_version().unwrap_or_default().contains("Windows"){

    //     let _ascii1 = r"⬛⬛⬛⬛  ";
    //     let _ascii2 = r"⬛⬛⬛⬛";
    //     let _ascii3 = r"⬛⬛⬛⬛       ;
    //     let _ascii4 = r"  ";
    //     let _ascii5 = r"       /  .-.  \    ";
    //     let _ascii6 = r"⬛⬛⬛⬛     /  (   ) _\   ";
    //     let _ascii7 = r"⬛⬛⬛⬛~   ~._^\  ";
    //     let _ascii8 = r"⬛⬛⬛⬛        ^.\ ";
    // }

    return retval;
}
fn main() {

    let os_ascii = ascii_art();
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
    
    for line in os_ascii.iter() {
        println!("{}", line);
    }

}

