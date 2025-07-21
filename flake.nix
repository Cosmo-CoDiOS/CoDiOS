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

  outputs = inputs: let
    inherit (inputs) self flake-utils fenix naersk nixpkgs;
  in
    flake-utils.lib.eachDefaultSystem (system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain = with fenix.packages.${system};
          fromToolchainFile {
            dir = ./.;
            sha256 = "sha256-QmNaeezyZvl+M+EIExXMhW60xi/LupjSwtQ1RFUBCuY=";
          };

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };

        naerskBuildPackage = target: args:
          naersk'.buildPackage (
            args
            // {CARGO_BUILD_TARGET = target;}
          );
      in rec
      {
        packages = {
          emulator-x86 = naerskBuildPackage "x86_64-unknown-linux-musl" rec {
            src = ./.;
            nativeBuildInputs = with pkgs; [pkgsStatic.stdenv.cc cmake];
            buildInputs = with pkgs; [SDL.dev SDL2.dev];
          };
          emulator-arm64 = naerskBuildPackage "aarch64-unknown-linux-musl" rec {
            src = ./.;
            nativeBuildInputs = with pkgs; [pkgsStatic.stdenv.cc cmake];
            buildInputs = with pkgs; [SDL.dev SDL2.dev];
          };
          firmware-official-codi = naerskBuildPackage "thumbv7em-none-eabihf" {
            src = ./.;
            nativeBuildInputs = with pkgs; [cmake];
          };
        };

        devShell =
          pkgs.mkShell
          {
            inputsFrom = with self.packages.${system}; [
              emulator-x86
              emulator-arm64
              firmware-official-codi
            ];
            packages = with pkgs; [
              rustup
              cargo-cross
            ];
            CROSS_CONTAINER_OPTS = "--platform linux/amd64";
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
          };
      });
}
