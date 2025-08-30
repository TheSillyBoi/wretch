use crate::coloring;
use sysinfo::System;
use colored::ColoredString;
use colored::Colorize;
use crate::Args;

pub fn content(args: &Args) -> [ColoredString; 8] { // Creates an array of the content shown

    let mut sys = System::new_all();
    sys.refresh_all(); let mut retval: [ColoredString; 8] = Default::default(); // Initialize an array of ColoredString with 8 elements
    let color = coloring::info_color(); // Calls the info_color function to get the color values

    retval[0] = format!(
        "{}'s System information",
        if args.username.is_some() {
            args.username
                .as_ref()
                .expect("args.username was Option::None, this should not happen please report this to us at https://github.com/thesillyboi/wretch/issues/new")
                .to_string()
        } else {
            whoami::realname()      
        }
    ).into();
    retval[1] = format!( 
        "System name: {} {}",
        System::name()
            .unwrap_or_else(|| "Unknown".to_string())
            .truecolor(color[0], color[1], color[2]),
        format!("({})", System::cpu_arch()).truecolor(color[0], color[1], color[2])
    ).into(); // Prints the system name and CPU architecture

    let mut hello = System::long_os_version().expect("it borked");
    remove_newlines(&mut hello);

    retval[2] = 
    format!(
        "Operating System Version: {}",
            hello
            .truecolor(color[0], color[1], color[2])
    ).into();


        retval[3] = format!(
        "System kernel version: {}",
        System::kernel_long_version().truecolor(color[0], color[1], color[2])).into()
    ; // Prints the kernel version


    retval[4] = format!(
        "Hostname: {}",
        format!("{}@{}", whoami::username(), System::host_name().unwrap())
            .truecolor(color[0], color[1], color[2])
    ).into();    

    // retval[5] = format!("Memory Usage: {}/{} MB", sys.used_memory() / 1024 / 1024, sys.total_memory() / 1024 / 1024).into();
    
    let mem_used_mb = sys.used_memory() / 1024 / 1024; // Converts used to MB
    let mem_total_mb = sys.total_memory() / 1024 / 1024; // Converts Total to MB
    let mem_usage_prc = sys.used_memory() * 100 / sys.total_memory(); // Calculates the percentage of memory used
    retval[5] = format!(
        "Memory Usage: {} {}",
        format!("{}/{} MB", mem_used_mb, mem_total_mb).truecolor(color[0], color[1], color[2]),
        format!("({}%)", mem_usage_prc).truecolor(color[0], color[1], color[2])
    ).into();
    
    
    
    if sys.total_swap() != 0 {
        let swap_used_mb = sys.used_swap() / 1024 / 1024; 
        let swap_total_mb = sys.total_swap() / 1024 / 1024;
        let swap_usage_prc = sys.used_swap() * 100 / sys.total_swap();


    retval[6] = format!(
            "Swap Usage: {} {}",
            format!("{}/{} MB", swap_used_mb, swap_total_mb)
                .truecolor(color[0], color[1], color[2]),
            format!("({}%)", swap_usage_prc).truecolor(color[0], color[1], color[2])
        ).into(); // prints the swap usage
    } else {
        retval[6] = format!(
            "Swap Usage: {}",
            "No Swap Memory".truecolor(color[0], color[1], color[2])
        ).into(); // prints if no swap memory is available
    }
    retval[7] = format!(
        "CPU Usage: {}",
        format!("{}%", sys.global_cpu_usage()).truecolor(color[0], color[1], color[2])
    ).into();

    retval
}


fn remove_newlines(var: &mut String){ //removes extra lines from System::long_os_version
	    if var.ends_with('\n'){
		    var.pop();
		    if var.ends_with('\r'){
			    var.pop();
		}
	}
}
