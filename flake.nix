
{
  description = "CBR to CBZ cli conversion tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.stable; # Use stable Rust from Fenix
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cbr-to-cbz";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ toolchain ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [ toolchain.cargo ];
        };
      }
    );
}
