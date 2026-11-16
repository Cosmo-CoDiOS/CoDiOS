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
            sha256 = "sha256-7xuGc7uu1WQSroZ0h7yCdrhr9eEFlEIcyoe8Hm8kp+g=";
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
          # NOTE: nixpkgs' SDL2 is marked unsupported for static linking
          # (badPlatforms = isStatic), so these link dynamically against a
          # musl-cross-built SDL2. That cross build has no cache.nixos.org
          # hits and compiles a large chunk of nixpkgs from source the
          # first time - expect this to take a long time (30+ minutes).
          emulator-x86 = naerskBuildPackage "x86_64-unknown-linux-musl" rec {
            src = ./.;
            nativeBuildInputs = with pkgs; [stdenv.cc cmake pkg-config];
            buildInputs = [pkgs.pkgsCross.musl64.SDL2];
            PKG_CONFIG_PATH = "${pkgs.pkgsCross.musl64.SDL2}/lib/pkgconfig";
            cargoBuildOptions = x: x ++ ["--features" "emulator"];
          };
          emulator-arm64 = naerskBuildPackage "aarch64-unknown-linux-musl" rec {
            src = ./.;
            nativeBuildInputs = with pkgs; [stdenv.cc cmake pkg-config];
            buildInputs = [pkgs.pkgsCross.aarch64-multiplatform-musl.SDL2];
            PKG_CONFIG_PATH = "${pkgs.pkgsCross.aarch64-multiplatform-musl.SDL2}/lib/pkgconfig";
            cargoBuildOptions = x: x ++ ["--features" "emulator"];
          };
          firmware-official-codi = naerskBuildPackage "thumbv7em-none-eabihf" {
            src = ./.;
            nativeBuildInputs = with pkgs; [cmake];
            cargoBuildOptions = x: x ++ ["--features" "firmware"];
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
            PKG_CONFIG_PATH = "${pkgs.pkgsCross.musl64.SDL2}/lib/pkgconfig";
            CROSS_CONTAINER_OPTS = "--platform linux/amd64";
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
          };
      });
}
