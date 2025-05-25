{
  description = "A simple Fetch CLI program Built with Rust";
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  outputs =
    { self, nixpkgs }:
    let 
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in{
      packages.x86_64-linux.default =
        with import nixpkgs {
          system = "x86_64-linux";
        };
        pkgs.rustPlatform.buildRustPackage rec {
          pname = "wretch";
          version = "nightly";

          src = self;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs;[
            cargo
            rustc
            pkgs.pkg-config
            git
          ];
        };
    };
}
