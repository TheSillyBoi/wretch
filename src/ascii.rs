use clap::Parser; // Import the Parser trait from clap
use colored::{Colorize, ColoredString}; // Import the Colorize trait and ColoredString struct from colored
use sysinfo::System; // Import the System struct from sysinfo
use std::sync::OnceLock; // Import OnceLock

use crate::is_generic; // Import the is_generic function from main.rs
use crate::get_os_name; // Import the get_os_name function from main.rs
use crate::Args; // Import the Args struct from main.rs
use crate::coloring::info_color;

pub fn art(os_ascii_name: &str) -> [ColoredString; 8] { // Function to Select an ASCII art based on the OS or parameter
    let mut retval: [ColoredString; 8] = Default::default(); // Initialize an array of ColoredString with 8 elements
    let colors = &info_color(); // Get the color values based on the OS
    if os_ascii_name.contains("arch"){ // if the OS version contains "arch"(meant to be used for arch linux)
        retval[0] = ColoredString::from(r"          ╔═╗       ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"         ╔╝ ╚╗      ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"        ╔╝   ╚╗     ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"       ╔╝     ╚╗    ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"      ╔╝ ╔═══╗ ╚╗   ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"     ╔╝  ║   ║  ╚╗  ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"    ╔╝╔══╝   ╚══╗╚╗ ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"    ╚═╝         ╚═╝ ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("fedora"){ // if the OS version contains "fedora" meant for fedora linux
        retval[0] = ColoredString::from(r"    ╔══════════╗    ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"  ╔═╝          ╚═╗  ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r" ╔╝     ╔═══╗    ╚╗ ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"╔╝      ║         ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"║  ╔═══ ╠═══      ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"║  ║    ║        ╔╝ ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"║  ╚════╝      ╔═╝  ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"╚══════════════╝    ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("windows"){ // if the OS version contains "windows" meant for windows
        retval[0] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(colors[0], colors[1], colors[2]);
} else if os_ascii_name.contains("nixos"){ // if the OS version contains "nix" meant for nixOS linux
        retval[0] = ColoredString::from(r"    ╚╗     ╚╗ ╔═    ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"     ╚╗     ╚╦╝     ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"  ════╩════  ╚╗  ╔  ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"   ╔╝         ╚ ╔╝  ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"══╦╝ ═╗        ╔╩══ ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r" ╔╝   ╚═╗     ╔╝    ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"      ╔═╣  ════╦═══ ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"     ═╝ ╚      ╚    ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("mac"){ // if the OS version contains "mac" meant for macOS
        retval[0] = ColoredString::from(r"           ╔═       ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"          ╔╝        ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"   ╔════╗   ╔═══╗   ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"  ╔╝    ╚═══╝  ╔╝   ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"  ║           ╔╝    ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"  ║           ╚╗    ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"  ╚╗   ╔════╗  ╚╗   ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"   ╚═══╝    ╚═══╝   ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("debian"){ // if the OS version contains "debian" meant for debian linux, to do
        retval[0] = ColoredString::from(r"        ╔═════╗     ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"      ╔═╝╔══╗ ║     ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"     ╔╝  ║ ╚╝╔╝     ").truecolor(colors[0], colors[1], colors[2]);  
        retval[3] = ColoredString::from(r"     ║   ╚═══╝      ").truecolor(colors[0], colors[1], colors[2]); 
        retval[4] = ColoredString::from(r"     ╚╗             ").truecolor(colors[0], colors[1], colors[2]); 
        retval[5] = ColoredString::from(r"      ╚═╗           ").truecolor(colors[0], colors[1], colors[2]); 
        retval[6] = ColoredString::from(r"        ╚═╗         ").truecolor(colors[0], colors[1], colors[2]); 
        retval[7] = ColoredString::from(r"          ╚═        ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("void"){ // if the OS version contains "void" meant for void linux
        retval[0] = ColoredString::from(r"        ╔═══════╗   ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"        ╚═════╗ ╚═╗ ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"              ╚═╗ ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"  ╔═╗    ╔═╗    ║ ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"  ║ ║    ╚═╝    ╚═╝ ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"  ║ ╚═╗             ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"  ╚═╗ ╚═════╗       ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"    ╚═══════╝       ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("suse"){ // if the OS version contains "suse" meant for opensuse linux
        retval[0] = ColoredString::from(r"      ╔═══════╗     ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"    ╔═╝╔══╗   ╚═╗   ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"  ╔═╩══╝  ╚═══╗ ╚═╗ ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"  ║        ╔═╗╚╗  ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"  ║        ╚═╝╔╝  ║ ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"  ╚═╗   ╔═════╝ ╔═╝ ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"    ╚═╦═╩════ ╔═╝   ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"      ╚═══════╝     ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("ubuntu"){ // if the OS version contains "ubuntu" meant for ubuntu linux
        retval[0] = ColoredString::from(r"             ╔═╗    ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"             ╚═╝    ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"    ╔══   ════╗     ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"╔═╗ ║  ╔════╗ ║     ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"╚═╝ ║  ║    ║ ║     ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"    ║  ╚════╝       ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"      ════════╝ ╔═╗ ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"                ╚═╝ ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("zorin"){ // if the OS version contains "zorin" meant for zorinOS linux
        retval[0] = ColoredString::from(r"    ____________    ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"   /____________\   ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"  _______           ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r" / _____/   _____/\ ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r" \/        /______/ ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"   ______________   ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"   \____________/   ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"                    ").truecolor(colors[0], colors[1], colors[2]);
    } else if os_ascii_name.contains("linux"){ // if the OS version contains "linux" meant for other linux distros
        retval[0] = ColoredString::from(r"       ╔════╗       ").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"      ╔╝○ ○ ╚╗      ").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"      ║ ╔══╗ ║      ").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"     ╔╝╔╩══╩╗╚╗     ").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"    ╔╝╔╝    ╚╗╚╗    ").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"  ╔═╩╗║      ║╔╩═╗  ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"  ║  ║╚══════╝║  ║  ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"  ╚══╩════════╩══╝  ").truecolor(colors[0], colors[1], colors[2]);
    } else { // if the OS version does not match any of the above
        retval[0] = ColoredString::from(r"╔══════════════════╗").truecolor(colors[0], colors[1], colors[2]);
        retval[1] = ColoredString::from(r"║                  ║").truecolor(colors[0], colors[1], colors[2]);
        retval[2] = ColoredString::from(r"║                  ║").truecolor(colors[0], colors[1], colors[2]);
        retval[3] = ColoredString::from(r"║                  ║").truecolor(colors[0], colors[1], colors[2]);
        retval[4] = ColoredString::from(r"╚═══════╦══╦═══════╝").truecolor(colors[0], colors[1], colors[2]);
        retval[5] = ColoredString::from(r"        ║  ║        ").truecolor(colors[0], colors[1], colors[2]);
        retval[6] = ColoredString::from(r"    ╔═══╝  ╚═══╗    ").truecolor(colors[0], colors[1], colors[2]);
        retval[7] = ColoredString::from(r"    ╚══════════╝    ").truecolor(colors[0], colors[1], colors[2]);
    }

    retval
}

pub fn name() -> String {
    static CACHE: OnceLock<String> = OnceLock::new();

    if let Some(cached) = CACHE.get() {
        return cached.clone();
    }

    let args = Args::parse();

    let result = if let Some(ascii_arg) = args.ascii {
        ascii_arg
    } else {
        let (id_opt, id_like_opt) = get_os_name();

        if let Some(ref id) = id_opt {
            let generic = is_generic(id);
            if !generic {
                id.clone()
            } else if let Some(ref id_like) = id_like_opt {
                id_like.clone()
            } else {
                let mut os_ascii_name =
                    System::long_os_version().unwrap_or_default().to_lowercase();
                os_ascii_name = os_ascii_name.replace(' ', "");
                os_ascii_name = os_ascii_name.replace('(', "");
                os_ascii_name = os_ascii_name.replace(')', "");
                os_ascii_name
            }
        } else if let Some(ref id_like) = id_like_opt {
            id_like.clone()
        } else {
            let mut os_ascii_name = System::long_os_version().unwrap_or_default().to_lowercase();
            os_ascii_name = os_ascii_name.replace(' ', "");
            os_ascii_name = os_ascii_name.replace('(', "");
            os_ascii_name = os_ascii_name.replace(')', "");
            os_ascii_name
        }
    };

    CACHE.set(result.clone()).ok();

    result
}