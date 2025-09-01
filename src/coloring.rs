use crate::ascii;

pub fn info_color() -> [u8; 3] {
    // Function to get the color values based on the OS for the Info
    let mut color = [1, 2, 3];
    let ascii_name = ascii::name();

    if ascii_name.contains("fedora")
        || ascii_name.contains("nixos")
        || ascii_name.contains("arch") {
        color[0] = 0;
        color[1] = 120;
        color[2] = 212;
    } else if ascii_name.contains("windows") 
        || ascii_name.contains("zorin") {
        color[0] = 0;
        color[1] = 0;
        color[2] = 250;
    } else if ascii_name.contains("ubuntu") {
        color[0] = 250;
        color[1] = 70;
        color[2] = 22;
    } else if ascii_name.contains("mac") {
        color[0] = 255;
        color[1] = 255;
        color[2] = 255;
    } else if ascii_name.contains("debian") {
        color[0] = 255;
        color[1] = 0;
        color[2] = 0;
    } else if ascii_name.contains("void") 
        || ascii_name.contains("suse")
        || ascii_name.contains("mint") {
        color[0] = 0;
        color[1] = 255;
        color[2] = 0;
    } else {
        color[0] = 255;
        color[1] = 255;
        color[2] = 255;
    }
    color
}