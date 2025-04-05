use sysinfo::{
    /*DiskExt, DiskType, NetworkExt, NetworksExt, ProcessExt, RefreshKind,*/ System,
    /*SystemExt*/
};


fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=>System information");
    println!("System name: {}", System::name().unwrap_or_default());
    println!("System kernel version: {}", System::kernel_version().unwrap_or_default());
    println!("Memory Usage : {}/{} MB ({}%)", sys.used_memory() / 1024 /1024, sys.total_memory() / 1024 / 1024, sys.used_memory() * 100 / sys.total_memory());
    if sys.total_swap() != 0 {
        println!("Swap Usage : {}/{} MB ({}%)", sys.used_swap() / 1024 /1024, sys.total_swap() / 1024 / 1024, sys.used_swap() * 100 / sys.total_swap());
    }
    println!("CPU Usage : {}%", sys.global_cpu_usage());

    }
