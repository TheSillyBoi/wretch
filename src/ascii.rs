use colored::{Colorize, ColoredString}; // Import the Colorize trait and ColoredString struct from colored
pub fn ascii_art(os_ascii_name: &str) -> ([ColoredString; 8], bool) { // Function to Select an ASCII art based on the OS or parameter
    let mut retval: [ColoredString; 8] = Default::default(); // Initialize an array of ColoredString with 8 elements
    let mut is_generic = false;
    if os_ascii_name.contains("arch"){ // if the OS version contains "arch"(meant to be used for arch linux)
        retval[0] = ColoredString::from(r"          ╔═╗       ").truecolor(0, 120, 212);
        retval[1] = ColoredString::from(r"         ╔╝ ╚╗      ").truecolor(0, 120, 212);
        retval[2] = ColoredString::from(r"        ╔╝   ╚╗     ").truecolor(0, 120, 212);
        retval[3] = ColoredString::from(r"       ╔╝     ╚╗    ").truecolor(0, 120, 212);
        retval[4] = ColoredString::from(r"      ╔╝ ╔═══╗ ╚╗   ").truecolor(0, 120, 212);
        retval[5] = ColoredString::from(r"     ╔╝  ║   ║  ╚╗  ").truecolor(0, 120, 212);
        retval[6] = ColoredString::from(r"    ╔╝╔══╝   ╚══╗╚╗ ").truecolor(0, 120, 212);
        retval[7] = ColoredString::from(r"    ╚═╝         ╚═╝ ").truecolor(0, 120, 212);
    } else if os_ascii_name.contains("fedora"){ // if the OS version contains "fedora" meant for fedora linux
        retval[0] = ColoredString::from(r"    ╔══════════╗    ").truecolor(0, 120, 212);
        retval[1] = ColoredString::from(r"  ╔═╝          ╚═╗  ").truecolor(0, 120, 212);
        retval[2] = ColoredString::from(r" ╔╝     ╔═══╗    ╚╗ ").truecolor(0, 120, 212);
        retval[3] = ColoredString::from(r"╔╝      ║         ║ ").truecolor(0, 120, 212);
        retval[4] = ColoredString::from(r"║  ╔═══ ╠═══      ║ ").truecolor(0, 120, 212);
        retval[5] = ColoredString::from(r"║  ║    ║        ╔╝ ").truecolor(0, 120, 212);
        retval[6] = ColoredString::from(r"║  ╚════╝      ╔═╝  ").truecolor(0, 120, 212);
        retval[7] = ColoredString::from(r"╚══════════════╝    ").truecolor(0, 120, 212);
    } else if os_ascii_name.contains("windows"){ // if the OS version contains "windows" meant for windows
        retval[0] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 0, 255);
        retval[1] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 0, 255);
        retval[2] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 0, 255);
        retval[3] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 0, 255);
        retval[4] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 0, 255);
        retval[5] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 0, 255);
        retval[6] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 0, 255);
        retval[7] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 0, 255);
} else if os_ascii_name.contains("nixos"){ // if the OS version contains "nix" meant for nixOS linux
        retval[0] = ColoredString::from(r"    ╚╗     ╚╗ ╔═    ").truecolor(0, 120, 212);
        retval[1] = ColoredString::from(r"     ╚╗     ╚╦╝     ").truecolor(0, 120, 212);
        retval[2] = ColoredString::from(r"  ════╩════  ╚╗  ╔  ").truecolor(0, 120, 212);
        retval[3] = ColoredString::from(r"   ╔╝         ╚ ╔╝  ").truecolor(0, 120, 212);
        retval[4] = ColoredString::from(r"══╦╝ ═╗        ╔╩══ ").truecolor(0, 120, 212);
        retval[5] = ColoredString::from(r" ╔╝   ╚═╗     ╔╝    ").truecolor(0, 120, 212);
        retval[6] = ColoredString::from(r"      ╔═╣  ════╦═══ ").truecolor(0, 120, 212);
        retval[7] = ColoredString::from(r"     ═╝ ╚      ╚    ").truecolor(0, 120, 212);
    } else if os_ascii_name.contains("mac"){ // if the OS version contains "mac" meant for macOS
        retval[0] = ColoredString::from(r"           ╔═       ").truecolor(255,255,255);
        retval[1] = ColoredString::from(r"          ╔╝        ").truecolor(255,255,255);
        retval[2] = ColoredString::from(r"   ╔════╗   ╔═══╗   ").truecolor(255,255,255);
        retval[3] = ColoredString::from(r"  ╔╝    ╚═══╝  ╔╝   ").truecolor(255,255,255);
        retval[4] = ColoredString::from(r"  ║           ╔╝    ").truecolor(255,255,255);
        retval[5] = ColoredString::from(r"  ║           ╚╗    ").truecolor(255,255,255);
        retval[6] = ColoredString::from(r"  ╚╗   ╔════╗  ╚╗   ").truecolor(255,255,255);
        retval[7] = ColoredString::from(r"   ╚═══╝    ╚═══╝   ").truecolor(255,255,255);
    } else if os_ascii_name.contains("debian"){ // if the OS version contains "debian" meant for debian linux, to do
        retval[0] = ColoredString::from(r"        ╔═════╗     ").truecolor(255,0,0);
        retval[1] = ColoredString::from(r"      ╔═╝╔══╗ ║     ").truecolor(255,0,0);
        retval[2] = ColoredString::from(r"     ╔╝  ║ ╚╝╔╝     ").truecolor(255,0,0);  
        retval[3] = ColoredString::from(r"     ║   ╚═══╝      ").truecolor(255,0,0); 
        retval[4] = ColoredString::from(r"     ╚╗             ").truecolor(255,0,0); 
        retval[5] = ColoredString::from(r"      ╚═╗           ").truecolor(255,0,0); 
        retval[6] = ColoredString::from(r"        ╚═╗         ").truecolor(255,0,0); 
        retval[7] = ColoredString::from(r"          ╚═        ").truecolor(255,0,0);
    } else if os_ascii_name.contains("void"){ // if the OS version contains "void" meant for void linux
        retval[0] = ColoredString::from(r"        ╔═══════╗   ").truecolor(0,255,0);
        retval[1] = ColoredString::from(r"        ╚═════╗ ╚═╗ ").truecolor(0,255,0);
        retval[2] = ColoredString::from(r"              ╚═╗ ║ ").truecolor(0,255,0);
        retval[3] = ColoredString::from(r"  ╔═╗    ╔═╗    ║ ║ ").truecolor(0,255,0);
        retval[4] = ColoredString::from(r"  ║ ║    ╚═╝    ╚═╝ ").truecolor(0,255,0);
        retval[5] = ColoredString::from(r"  ║ ╚═╗             ").truecolor(0,255,0);
        retval[6] = ColoredString::from(r"  ╚═╗ ╚═════╗       ").truecolor(0,255,0);
        retval[7] = ColoredString::from(r"    ╚═══════╝       ").truecolor(0,255,0);
    } else if os_ascii_name.contains("suse"){ // if the OS version contains "suse" meant for opensuse linux
        retval[0] = ColoredString::from(r"      ╔═══════╗     ").truecolor(0,255,0);
        retval[1] = ColoredString::from(r"    ╔═╝╔══╗   ╚═╗   ").truecolor(0,255,0);
        retval[2] = ColoredString::from(r"  ╔═╩══╝  ╚═══╗ ╚═╗ ").truecolor(0,255,0);
        retval[3] = ColoredString::from(r"  ║        ╔═╗╚╗  ║ ").truecolor(0,255,0);
        retval[4] = ColoredString::from(r"  ║        ╚═╝╔╝  ║ ").truecolor(0,255,0);
        retval[5] = ColoredString::from(r"  ╚═╗   ╔═════╝ ╔═╝ ").truecolor(0,255,0);
        retval[6] = ColoredString::from(r"    ╚═╦═╩════ ╔═╝   ").truecolor(0,255,0);
        retval[7] = ColoredString::from(r"      ╚═══════╝     ").truecolor(0,255,0);
    } else if os_ascii_name.contains("ubuntu"){ // if the OS version contains "ubuntu" meant for ubuntu linux
        retval[0] = ColoredString::from(r"             ╔═╗    ").truecolor(250, 70, 22);
        retval[1] = ColoredString::from(r"             ╚═╝    ").truecolor(250, 70, 22);
        retval[2] = ColoredString::from(r"    ╔══   ════╗     ").truecolor(250, 70, 22);
        retval[3] = ColoredString::from(r"╔═╗ ║  ╔════╗ ║     ").truecolor(250, 70, 22);
        retval[4] = ColoredString::from(r"╚═╝ ║  ║    ║ ║     ").truecolor(250, 70, 22);
        retval[5] = ColoredString::from(r"    ║  ╚════╝       ").truecolor(250, 70, 22);
        retval[6] = ColoredString::from(r"      ════════╝ ╔═╗ ").truecolor(250, 70, 22);
        retval[7] = ColoredString::from(r"                ╚═╝ ").truecolor(250, 70, 22);
    } else if os_ascii_name.contains("zorin"){ // if the OS version contains "zorin" meant for zorinOS linux
        retval[0] = ColoredString::from(r"    ____________    ").truecolor(0, 0, 255);
        retval[1] = ColoredString::from(r"   /____________\   ").truecolor(0, 0, 255);
        retval[2] = ColoredString::from(r"  _______           ").truecolor(0, 0, 255);
        retval[3] = ColoredString::from(r" / _____/   _____/\ ").truecolor(0, 0, 255);
        retval[4] = ColoredString::from(r" \/        /______/ ").truecolor(0, 0, 255);
        retval[5] = ColoredString::from(r"   ______________   ").truecolor(0, 0, 255);
        retval[6] = ColoredString::from(r"   \____________/   ").truecolor(0, 0, 255);
        retval[7] = ColoredString::from(r"                    ").truecolor(0, 0, 255);
    } else if os_ascii_name.contains("linux"){ // if the OS version contains "linux" meant for other linux distros
        is_generic = true;
        retval[0] = ColoredString::from(r"       ╔════╗       ").white();
        retval[1] = ColoredString::from(r"      ╔╝○ ○ ╚╗      ").white();
        retval[2] = ColoredString::from(r"      ║ ╔══╗ ║      ").white();
        retval[3] = ColoredString::from(r"     ╔╝╔╩══╩╗╚╗     ").white();
        retval[4] = ColoredString::from(r"    ╔╝╔╝    ╚╗╚╗    ").white();
        retval[5] = ColoredString::from(r"  ╔═╩╗║      ║╔╩═╗  ").white();
        retval[6] = ColoredString::from(r"  ║  ║╚══════╝║  ║  ").white();
        retval[7] = ColoredString::from(r"  ╚══╩════════╩══╝  ").white();
    } else { // if the OS version does not match any of the above
        is_generic = true;
        retval[0] = ColoredString::from(r"╔══════════════════╗").truecolor(255,255,255);
        retval[1] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[2] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[3] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[4] = ColoredString::from(r"╚═══════╦══╦═══════╝").truecolor(255,255,255);
        retval[5] = ColoredString::from(r"        ║  ║        ").truecolor(255,255,255);
        retval[6] = ColoredString::from(r"    ╔═══╝  ╚═══╗    ").truecolor(255,255,255);
        retval[7] = ColoredString::from(r"    ╚══════════╝    ").truecolor(255,255,255);
        
    }

    (retval, is_generic)
}