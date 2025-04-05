use sysinfo::{
    /*DiskExt, DiskType, NetworkExt, NetworksExt, ProcessExt, RefreshKind,*/ System,
    /*SystemExt*/
};


fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=>System information");
    println!("System name: {}", System::name().unwrap_or_default());
}
