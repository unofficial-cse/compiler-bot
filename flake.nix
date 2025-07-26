{
  description = "Flake for Unofficial.CSE compiler-bot";

  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        toolchain = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchin;
          sha256 = "";
        }).rust;

        naersk-lib = pkgs.callPackage naersk {
          cargo = toolchain;
          clippy = toolchain;
          rustc = toolchain;
          rustfmt = toolchain;
        };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ toolchain ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
