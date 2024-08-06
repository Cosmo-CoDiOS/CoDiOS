# SPDX-FileCopyrightText: 2024 The Cosmo-CoDiOS Developers
#
# SPDX-License-Identifier: GPL-3.0-only

{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain = with fenix.packages.${system}; fromToolchainFile {
          dir = ./.;
          sha256 = "sha256-oW7iyYzGcgW5TjRA2HLhYVW2WNTNadIe4SX7IWsrs3g=";
        };

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };

        naerskBuildPackage = target: args:
          naersk'.buildPackage (
            args
            // { CARGO_BUILD_TARGET = target; }
          );
      in
      rec
      {
        defaultPackage = packages.x86_64-unknown-linux-musl;

        # For `nix build .#x86_64-unknown-linux-musl`:
        packages.x86_64-unknown-linux-musl = naerskBuildPackage "x86_64-unknown-linux-musl" rec {
          src = ./.;
          doCheck = true;
          nativeBuildInputs = with pkgs; [ pkgsStatic.stdenv.cc cmake SDL SDL2 SDL.dev SDL2.dev ];
          buildInputs = nativeBuildInputs;
        };

        # For `nix build .#x86_64-unknown-linux-musl`:
        packages.aarch64-unknown-linux-musl = naerskBuildPackage "aarch64-unknown-linux-musl" rec {
          src = ./.;
          doCheck = true;
          nativeBuildInputs = with pkgs; [ pkgsStatic.stdenv.cc cmake SDL SDL2 SDL.dev SDL2.dev ];
          buildInputs = nativeBuildInputs;
        };

        # For `nix build .#x86_64-unknown-linux-musl`:
        packages.thumbv7em-none-eabihf = naerskBuildPackage "thumbv7em-none-eabihf" {
          src = ./.;
          doCheck = true;
          nativeBuildInputs = with pkgs; [ cmake ];
        };

        devShell = pkgs.mkShellNoCC (
          {
            inputsFrom = [ packages.x86_64-unknown-linux-musl packages.aarch64-unknown-linux-musl ];
            packages = with pkgs; [
              rustup
              cargo-cross
            ];
            CROSS_CONTAINER_OPTS = "--platform linux/amd64";
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
          });
      });
}
