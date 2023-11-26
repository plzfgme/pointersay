{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };
      in
      rec {
        packages = {
          default = naersk'.buildPackage {
            pname = "pointersay";
            version = "0.1.0";

            src = ./.;

            buildInputs = with pkgs; [ libxkbcommon glib pango gdk-pixbuf graphene gtk4 gtk4-layer-shell ];
            nativeBuildInputs = with pkgs; [ pkg-config ];
          };
        };

        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = packages.default.buildInputs ++ packages.default.nativeBuildInputs;
          };
        };
      }
    );
}
