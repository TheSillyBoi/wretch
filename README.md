# Wretch
a fetch type cli program that tells you info about ur system

### Why in the hell does this exist
- We wanted to learn Rust cause it cool and speedy vroom

### Why is the code so bad and horrible
- we are starting out at this, feel free to contribute

## Installation
Right now you can get the [latest release](https://github.com/thesillyboi/wretch/releases/latest)<br />
> [!IMPORTANT]
> **macOS users**: There is only a Apple Silicon build for wretch and **NOT** for Intel Macs (should come soon when i can make github actions do that)

### Arch Linux
**For Arch Linux Users** you can get the package from the AUR using a aur package manager like [yay](https://github.com/Jguer/yay) for example or one of your choice<br />
> To install using yay run the following command
>```shell
>yay -S wretch
>```
**Arch Linux Users** you can also get the package via the AUR via these steps
1. Install `git` and `base-devel` packages
```shell
sudo pacman -S git base-devel
```
2. Clone the repository from the AUR and change directory to it
```shell
git clone https://aur.archlinux.org/wretch && cd wretch
```
3. Install!
```shell
makepkg -si
```
> [!NOTE]
> The `s` flag specifies to install dependecies that are not installed yet on your system but are required by the program.<br />
> The `i` flag specifies the makepkg program to run the install script after building to install it onto your system!

## Building and Compiling 
 - Download `rustup` from your package manager or at https://rustup.rs/ if it's not available.
> [!IMPORTANT]
   > Don't use rust from your package manager,
   > use rustup instead<br />
   > If you're on Windows or your distro doesn't have rustup in your package manager use https://rustup.rs/
 - run `rustup update stable` to get the latest toolchain
 - Clone Repo `git clone https://github.com/thesillyboi/wretch`
 - Run `cargo run --release` to run and build the best version of the project
> - *optionally* (on Linux/Mac) run `sudo mv target/release/wretch /usr/local/bin/wretch` so you can call `wretch` without defining the path

> - *optionally* (on Windows) to add to your `PATH` environment variable and run without defining the path everytime you want to run it
>   - You can move it to any folder you want
>     - I recommend making a folder for cli programs downloaded not from a package manager. like a folder in your documents.
>   - Then open `Start` or the search and type something like `env` or `environment` and open `Edit the system environment variables` Control Panel Tool.
>   - If you are not already make sure you're in the `Advanced` tab
>   - At the bottom right of the window click `Environment Variables...`
>   - In the `User variables for [your username]` section, scroll down until you see something named `Path` in the `Variable` column.
>   - Either double click on that item or click on it once and then click `Edit...` in the User varuables section>>   - Now click on `New` on the window that popped up
>   - Type in the path where your wretch lies but, leave out the wretch as this will include each .cmd, .bat, and .exe file to be in your path. (It also doesn't go into subdirectories)
>     - Example: C:\Users\username\Documents\command-line-programs
>     - Example: C:\Users\username\OneDrive\Documents\command-line-programs
>   - Now click `OK` then click `OK` again and `OK` once more
>   - Now restart/close any shell/cmd/pwsh/powershells you have open and reopen them
>   - Now try running `wretch`
>   - Instead of doing the above two steps you can restart your computer instead

## Credits
https://www.asciiart.eu/computers/linux for ascii art of some logos
