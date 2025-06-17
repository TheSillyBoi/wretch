# installation
<a href="https://repology.org/project/wretch/versions">
    <img src="https://repology.org/badge/vertical-allrepos/wretch.svg" alt="repology packaging status for wretch" align="right">
</a>
look at the right to see the packages that have the latest version

## winget (recommended for windows)
1. run `winget install TheSillyBoi.Wretch`
2. restart shell

## manually
1. get from latest release
2. if on macOS or linux do `chmod +x ./wretch-v#.#.#-platform`
3. run it from the command line

## cargo (basically building from source)
1. install cargo via [rustup](https://rustup.rs/)
2. run `cargo install wretch`

## linux
### arch based (using aur)
1. use yay or another aur package manager
2. `yay -Syu wretch`
or
1. install `git` and `base-devel` packages. *you likely already have these packages, but it's good to check.*
```shell
sudo pacman -S git base-devel
```
2. clone the repository from the aur and change directory to it
```shell
git clone https://aur.archlinux.org/wretch && cd wretch
```
3. install!
```shell
makepkg -si
```
> [!NOTE]
> the `s` flag specifies to install dependecies that are not installed yet on your system but are required by the program.<br />
> the `i` flag specifies the makepkg program to run the install script after building to install it onto your system!

### nix (+ macOS if using nix flakes)
**nix users** you can get the pacakge via nix flakes. 
1. add the url to your ```flake.nix``` input
```nix
wretch.url = "github:thesillyboi/wretch";
```
2. add the pacakge in ```environment.systemPackages```
```nix
inputs.wretch.packages."${system}".default
```
3. rebuild your configuration with nix flakes enabled.
> [!TIP]
> update the application using:
> ```nix
> nix flake update wretch
> ```
