use colored::{Colorize, ColoredString}; // Import the Colorize trait and ColoredString struct from colored
use sysinfo::System; // Import the System struct from sysinfo
pub fn ascii_art() -> [ColoredString; 8] { // Function to Select an ASCII art based on the OS
    let os_version = System::long_os_version().unwrap_or_default().to_lowercase(); // Get the OS version and convert it to lowercase
    let mut retval: [ColoredString; 8] = Default::default(); // Initialize an array of ColoredString with 8 elements
    if os_version.contains("arch"){ // if the OS version contains "arch"(meant to be used for arch linux)
        retval[0] = ColoredString::from(r"           .        ").bright_cyan();
        retval[1] = ColoredString::from(r"          / \       ").bright_cyan();
        retval[2] = ColoredString::from(r"         /   \      ").bright_cyan();
        retval[3] = ColoredString::from(r"        /^.   \     ").bright_cyan();
        retval[4] = ColoredString::from(r"       /  .-.  \    ").bright_cyan();
        retval[5] = ColoredString::from(r"      /  (   ) _\   ").bright_cyan();
        retval[6] = ColoredString::from(r"     / _.~   ~._^\  ").bright_cyan();
        retval[7] = ColoredString::from(r"    /.^         ^.\ ").bright_cyan();
    } else if os_version.contains("fedora"){ // if the OS version contains "fedora" meant for fedora linux
        retval[0] = ColoredString::from(r"    ╔══════════╗    ").bright_cyan();
        retval[1] = ColoredString::from(r"  ╔═╝          ╚═╗  ").bright_cyan();
        retval[2] = ColoredString::from(r" ╔╝     ╔═══╗    ╚╗ ").bright_cyan();
        retval[3] = ColoredString::from(r"╔╝      ║         ║ ").bright_cyan();
        retval[4] = ColoredString::from(r"║  ╔═══ ╠═══      ║ ").bright_cyan();
        retval[5] = ColoredString::from(r"║  ║    ║        ╔╝ ").bright_cyan();
        retval[6] = ColoredString::from(r"║  ╚════╝      ╔═╝  ").bright_cyan();
        retval[7] = ColoredString::from(r"╚══════════════╝    ").bright_cyan();
    } else if os_version.contains("windows"){ // if the OS version contains "windows" meant for windows
        retval[0] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 120, 212);
        retval[1] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[2] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[3] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 120, 212);
        retval[4] = ColoredString::from(r" ╔══════╗  ╔══════╗ ").truecolor(0, 120, 212);
        retval[5] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[6] = ColoredString::from(r" ║      ║  ║      ║ ").truecolor(0, 120, 212);
        retval[7] = ColoredString::from(r" ╚══════╝  ╚══════╝ ").truecolor(0, 120, 212);
    } else if os_version.contains("mac"){ // if the OS version contains "mac" meant for macOS
        retval[0] = ColoredString::from(r"           ╔═       ").white();
        retval[1] = ColoredString::from(r"          ╔╝        ").white();
        retval[2] = ColoredString::from(r"   ╔════╗   ╔═══╗   ").white();
        retval[3] = ColoredString::from(r"  ╔╝    ╚═══╝  ╔╝   ").white();
        retval[4] = ColoredString::from(r"  ║           ╔╝    ").white();
        retval[5] = ColoredString::from(r"  ║           ╚╗    ").white();
        retval[6] = ColoredString::from(r"  ╚╗   ╔════╗  ╚╗   ").white();
        retval[7] = ColoredString::from(r"   ╚═══╝    ╚═══╝   ").white();
    } else if os_version.contains("debian"){ // if the OS version contains "debian" meant for debian linux, to do
        retval[0] = ColoredString::from(r"        ╔═════╗     ").red();
        retval[1] = ColoredString::from(r"      ╔═╝╔══╗ ║     ").red();
        retval[2] = ColoredString::from(r"     ╔╝  ║ ╚╝╔╝     ").red();  
        retval[3] = ColoredString::from(r"     ║   ╚═══╝      ").red(); 
        retval[4] = ColoredString::from(r"     ╚╗             ").red(); 
        retval[5] = ColoredString::from(r"      ╚═╗           ").red(); 
        retval[6] = ColoredString::from(r"        ╚═╗         ").red(); 
        retval[7] = ColoredString::from(r"          ╚═        ").red();
    } else if os_version.contains("void"){ // if the OS version contains "void" meant for void linux
        retval[0] = ColoredString::from(r"       ╔═══════╗    ").green();
        retval[1] = ColoredString::from(r"       ╚═════╗ ╚═╗  ").green();
        retval[2] = ColoredString::from(r"             ╚═╗ ║  ").green();
        retval[3] = ColoredString::from(r"   ╔═╗   ╔═╗   ║ ║  ").green();
        retval[4] = ColoredString::from(r"   ║ ║   ╚═╝   ╚═╝  ").green();
        retval[5] = ColoredString::from(r"   ║ ╚═╗            ").green();
        retval[6] = ColoredString::from(r"   ╚═╗ ╚═════╗      ").green();
        retval[7] = ColoredString::from(r"     ╚═══════╝      ").green();
    } else if os_version.contains("suse"){ // if the OS version contains "suse" meant for opensuse linux
        retval[0] = ColoredString::from(r"      ╔═══════╗     ").green();
        retval[1] = ColoredString::from(r"    ╔═╝╔══╗   ╚═╗   ").green();
        retval[2] = ColoredString::from(r"  ╔═╩══╝  ╚═══╗ ╚═╗ ").green();
        retval[3] = ColoredString::from(r"  ║        ╔═╗╚╗  ║ ").green();
        retval[4] = ColoredString::from(r"  ║        ╚═╝╔╝  ║ ").green();
        retval[5] = ColoredString::from(r"  ╚═╗   ╔═════╝ ╔═╝ ").green();
        retval[6] = ColoredString::from(r"    ╚═╦═╩════ ╔═╝   ").green();
        retval[7] = ColoredString::from(r"      ╚═══════╝     ").green();
    } else if os_version.contains("ubuntu"){ // if the OS version contains "ubuntu" meant for ubuntu linux
        retval[0] = ColoredString::from(r"             ╔═╗    ").truecolor(250, 70, 22);
        retval[1] = ColoredString::from(r"             ╚═╝    ").truecolor(250, 70, 22);
        retval[2] = ColoredString::from(r"    ╔══   ════╗     ").truecolor(250, 70, 22);
        retval[3] = ColoredString::from(r"╔═╗ ║  ╔════╗ ║     ").truecolor(250, 70, 22);
        retval[4] = ColoredString::from(r"╚═╝ ║  ║    ║ ║     ").truecolor(250, 70, 22);
        retval[5] = ColoredString::from(r"    ║  ╚════╝       ").truecolor(250, 70, 22);
        retval[6] = ColoredString::from(r"      ════════╝ ╔═╗ ").truecolor(250, 70, 22);
        retval[7] = ColoredString::from(r"                ╚═╝ ").truecolor(250, 70, 22);
    } else if os_version.contains("zorin"){ // if the OS version contains "zorin" meant for zorinOS linux
        retval[0] = ColoredString::from(r"    ____________    ").blue();
        retval[1] = ColoredString::from(r"   /____________\   ").blue();
        retval[2] = ColoredString::from(r"  _______           ").blue();
        retval[3] = ColoredString::from(r" / _____/   _____/\ ").blue();
        retval[4] = ColoredString::from(r" \/        /______/ ").blue();
        retval[5] = ColoredString::from(r"   ______________   ").blue();
        retval[6] = ColoredString::from(r"   \____________/   ").blue();
        retval[7] = ColoredString::from(r"                    ").blue();
    } else if os_version.contains("linux"){ // if the OS version contains "linux" meant for other linux distros
        retval[0] = ColoredString::from(r"        .---.       ");
        retval[1] = ColoredString::from(r"       /     \      ");
        retval[2] = ColoredString::from(r"       \.o-o./      ");
        retval[3] = ColoredString::from(r"       /`\_/`\      ");
        retval[4] = ColoredString::from(r"      //     \\     ");
        retval[5] = ColoredString::from(r"     | \     )|_    ");
        retval[6] = ColoredString::from(r"    /`\_`>  <_/ \   ");
        retval[7] = ColoredString::from(r" jgs\__/'---'\__/   ");
    } else { // if the OS version does not match any of the above
        retval[0] = ColoredString::from(r"╔══════════════════╗").truecolor(255,255,255);
        retval[1] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[2] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[3] = ColoredString::from(r"║                  ║").truecolor(255,255,255);
        retval[4] = ColoredString::from(r"╚═══════╦══╦═══════╝").truecolor(255,255,255);
        retval[5] = ColoredString::from(r"        ║  ║        ").truecolor(255,255,255);
        retval[6] = ColoredString::from(r"    ╔═══╝  ╚═══╗    ").truecolor(255,255,255);
        retval[7] = ColoredString::from(r"    ╚══════════╝    ").truecolor(255,255,255);
      
    }

    retval
}