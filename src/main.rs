use sysinfo::{
    DiskExt, DiskType, NetworkExt, NetworksExt, ProcessExt, RefreshKind, System,
    SystemExt,
};

let mut sys = System::new_all();

sys.refresh_all();

println!("=>System information");
println!("System name: {}", sys.name().unwrap_or_default());
