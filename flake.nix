{
  description = "A simple Fetch CLI program Built with Rust";
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.rustPlatform.buildRustPackage rec {
            pname = "wretch";
            version = "nightly";

            src = self;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              cargo
              rustc
              pkgs.pkg-config
              git
            ];
          };
        }
      );
    };
}
