# Wretch
a fetch-type CLI program that tells you info about ur system

### Why in the hell does this exist
- We wanted to learn Rust cause it cool and speedy vroom

### Why is the code so bad and horrible
- We are starting out at this, feel free to contribute(if you're looking for something to contribute, check TODO.md)
## Installation
Right now you can get the [latest release](https://github.com/thesillyboi/wretch/releases/latest)<br />
> [!IMPORTANT]
> **macOS users**: There is only an Apple Silicon build for wretch and **NOT** for Intel Macs (should come soon when I can make GitHub Actions do that)

### Arch Linux
**For Arch Linux Users** you can get the package from the AUR using an AUR package manager like [yay](https://github.com/Jguer/yay), for example, or one of your choice<br />
> To install using yay, run the following command
>```shell
>yay -S wretch
>```
**Arch Linux Users**, you can also get the package via the AUR without the use of a AUR package manager via these steps
1. Install `git` and `base-devel` packages*You likely already have these packages, but it's good to check*
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

### NixOS Linux
**NixOS Users** you can get the pacakge via Nix flakes. 
1. Add the url to your ```flake.nix``` input
```nix
wretch.url = "github:thesillyboi/wretch";
```
2. Add the pacakge in ```environment.systemPackages```
```nix
inputs.wretch.packages."${system}".default
```
3. Rebuild your configuration with nix flakes enabled.
> [!TIP]
> Update the application using:
> ```nix
> nix flake update wretch
> ```

## Building and Compiling
   *Ignore the current paragraph if you already have rustup installed and working*
 - Download `rustup` from your package manager or at https://rustup.rs/ if it's not available.
> [!IMPORTANT]
   > Don't use rust from your package manager,
   > use rustup instead<br />
   > If you're on Windows or your distro doesn't have rustup in your package manager use https://rustup.rs/
 - run `rustup update stable` to get the latest toolchain

 - Clone Repo `git clone https://github.com/thesillyboi/wretch`
 - Run `cargo run --release` to run and build the best version of the project


#### To get added to PATH(not needing to define where the file is)
 -  (on Linux/Mac) run `sudo mv target/release/wretch /usr/local/bin/wretch` so you can call `wretch` without defining the path *(or move it to another folder in the PATH, such as /bin, /USR/BIN/, /SBIN/, /USR/SBIN, or you can put another folder in the PATH)*

 - (on Windows) to add to your `PATH` environment variable and run without defining the path everytime you want to run it
   - You can move it to any folder you want
     - I recommend making a folder for cli programs downloaded not from a package manager. like a folder in your documents.
   - Then open `Start` or the search and type something like `env` or `environment` and open `Edit the system environment variables` Control Panel Tool.
   - If you are not already make sure you're in the `Advanced` tab
   - At the bottom right of the window click `Environment Variables...`
   - In the `User variables for [your username]` section, scroll down until you see something named `Path` in the `Variable` column.
   - Either double click on that item or click on it once and then click `Edit...` in the User varuables section>>   - Now click on `New` on the window that popped up
   - Type in the path where your wretch lies, but leave out the wretch as this will include each .cmd, .bat, and .exe file to be in your path. (It also doesn't go into subdirectories)
     - Example: C:\Users\username\Documents\command-line-programs
     - Example: C:\Users\username\OneDrive\Documents\command-line-programs
   - Now click `OK`, then click `OK` again, and `OK` once more
   - Now, restart/close any shells/cmd/pwsh/powershells you have open and reopen them
   - Now try running `wretch`
   - Instead of doing the above two steps, you can restart your computer.

## Credits
All logos were created by Adrian Tennies, if you want your distro added/you think you can do it better, please keep the art style intact(trying to make it look like a neon sign), and put it in a pull request, and modify this to credit yourself for the logo
